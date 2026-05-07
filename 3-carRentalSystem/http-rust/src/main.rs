use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize)]
struct Booking {
    bookingId: u64,
    carName: String,
    days: u64,
    rentPerDay: u64,
    status: String,
    #[serde(skip_serializing)]
    totalCost: u64,
}

#[derive(Debug, Clone, Serialize)]
struct User {
    id: u64,
    username: String,
    password: String,
    bookings: Vec<Booking>,
}

#[derive(Debug)]
struct AppState {
    global_id: Mutex<u64>,
    global_booking_id: Mutex<u64>,
    users: Mutex<Vec<User>>,
}

fn booking_public(b: &Booking) -> Value {
    json!({
        "bookingId": b.bookingId,
        "carName": b.carName,
        "days": b.days,
        "rentPerDay": b.rentPerDay,
        "status": b.status,
    })
}

async fn signup(body: web::Json<Value>, state: web::Data<AppState>) -> impl Responder {
    let username = match body.get("username").and_then(|v| v.as_str()) {
        Some(v) => v.to_string(),
        None => return HttpResponse::BadRequest().json(json!({ "message": "invalid data" })),
    };
    let password = match body.get("password").and_then(|v| v.as_str()) {
        Some(v) => v.to_string(),
        None => return HttpResponse::BadRequest().json(json!({ "message": "invalid data" })),
    };

    let mut users = state.users.lock().unwrap();
    if users.iter().any(|u| u.username == username) {
        return HttpResponse::Unauthorized().json(json!({ "message": "user already exist" }));
    }
    let mut gid = state.global_id.lock().unwrap();
    let id = *gid;
    *gid += 1;
    users.push(User {
        id,
        username,
        password,
        bookings: Vec::new(),
    });
    HttpResponse::Created().json(json!({ "message": "User created successfully", "userId": id }))
}

async fn list_users(state: web::Data<AppState>) -> impl Responder {
    let users = state.users.lock().unwrap();
    let arr: Vec<Value> = users
        .iter()
        .map(|u| {
            json!({
                "id": u.id,
                "username": u.username,
                "password": u.password,
                "bookings": u.bookings.iter().map(booking_public).collect::<Vec<_>>()
            })
        })
        .collect();
    HttpResponse::Ok().json(json!({ "users": arr }))
}

#[derive(Deserialize)]
struct BookingCreate {
    carName: String,
    days: u64,
    rentPerDay: u64,
}

async fn create_booking(
    path: web::Path<String>,
    state: web::Data<AppState>,
    body: web::Json<BookingCreate>,
) -> impl Responder {
    let user_id: u64 = match path.into_inner().parse() {
        Ok(v) => v,
        Err(_) => return HttpResponse::NotFound().json(json!({ "message": "user not found" })),
    };
    let mut users = state.users.lock().unwrap();
    let user = match users.iter_mut().find(|u| u.id == user_id) {
        Some(u) => u,
        None => return HttpResponse::NotFound().json(json!({ "message": "user not found" })),
    };
    let mut gbid = state.global_booking_id.lock().unwrap();
    let bid = *gbid;
    *gbid += 1;
    let total_cost = body.days * body.rentPerDay;
    user.bookings.push(Booking {
        bookingId: bid,
        carName: body.carName.clone(),
        days: body.days,
        rentPerDay: body.rentPerDay,
        status: "booked".into(),
        totalCost: total_cost,
    });
    HttpResponse::Created().json(json!({
        "message": format!("{} booked", body.carName),
        "bookingId": bid,
        "totalCost": total_cost
    }))
}

async fn get_user_bookings(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {
    let user_id: u64 = match path.into_inner().parse() {
        Ok(v) => v,
        Err(_) => return HttpResponse::NotFound().json(json!({ "message": "user not found" })),
    };
    let users = state.users.lock().unwrap();
    let user = match users.iter().find(|u| u.id == user_id) {
        Some(u) => u,
        None => return HttpResponse::NotFound().json(json!({ "message": "user not found" })),
    };
    HttpResponse::Ok().json(json!({
        "bookings": user.bookings.iter().map(booking_public).collect::<Vec<_>>()
    }))
}

async fn get_specific_booking(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> impl Responder {
    let (uid, bid) = path.into_inner();
    let user_id: u64 = uid.parse().unwrap_or(0);
    let booking_id: u64 = bid.parse().unwrap_or(0);
    let users = state.users.lock().unwrap();
    let booking = users
        .iter()
        .find(|u| u.id == user_id)
        .and_then(|u| u.bookings.iter().find(|b| b.bookingId == booking_id));
    match booking {
        Some(b) => HttpResponse::Ok().json(booking_public(b)),
        None => HttpResponse::NotFound().json(json!({ "message": "booking not found" })),
    }
}

#[derive(Deserialize)]
struct UpdateBody {
    carName: Option<String>,
    days: Option<u64>,
    rentPerDay: Option<u64>,
}

async fn update_booking(
    path: web::Path<(String, String)>,
    body: web::Json<UpdateBody>,
    state: web::Data<AppState>,
) -> impl Responder {
    let (uid, bid) = path.into_inner();
    let user_id: u64 = uid.parse().unwrap_or(0);
    let booking_id: u64 = bid.parse().unwrap_or(0);
    let mut users = state.users.lock().unwrap();
    let booking = users
        .iter_mut()
        .find(|u| u.id == user_id)
        .and_then(|u| u.bookings.iter_mut().find(|b| b.bookingId == booking_id));
    match booking {
        Some(b) => {
            if let Some(c) = &body.carName {
                b.carName = c.clone();
            }
            if let Some(d) = body.days {
                b.days = d;
            }
            if let Some(r) = body.rentPerDay {
                b.rentPerDay = r;
            }
            b.totalCost = b.days * b.rentPerDay;
            HttpResponse::Ok().json(booking_public(b))
        }
        None => HttpResponse::NotFound().json(json!({ "message": "booking not found" })),
    }
}

#[derive(Deserialize)]
struct StatusBody {
    status: String,
}

async fn update_status(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
    body: web::Json<StatusBody>,
) -> impl Responder {
    let (uid, bid) = path.into_inner();
    let user_id: u64 = uid.parse().unwrap_or(0);
    let booking_id: u64 = bid.parse().unwrap_or(0);
    let mut users = state.users.lock().unwrap();
    let booking = users
        .iter_mut()
        .find(|u| u.id == user_id)
        .and_then(|u| u.bookings.iter_mut().find(|b| b.bookingId == booking_id));
    match booking {
        Some(b) => {
            b.status = body.status.clone();
            HttpResponse::Ok().json(json!({ "message": "Status updated successfully" }))
        }
        None => HttpResponse::NotFound().json(json!({ "message": "booking not found" })),
    }
}

async fn delete_booking(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> impl Responder {
    let (uid, bid) = path.into_inner();
    let user_id: u64 = uid.parse().unwrap_or(0);
    let booking_id: u64 = bid.parse().unwrap_or(0);
    let mut users = state.users.lock().unwrap();
    let user = match users.iter_mut().find(|u| u.id == user_id) {
        Some(u) => u,
        None => return HttpResponse::NotFound().json(json!({ "message": "user not found" })),
    };
    let before = user.bookings.len();
    user.bookings.retain(|b| b.bookingId != booking_id);
    if user.bookings.len() < before {
        HttpResponse::Ok().json(json!({ "message": "Booking deleted successfully" }))
    } else {
        HttpResponse::NotFound().json(json!({ "message": "booking not found" }))
    }
}

async fn summary(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let user_id: u64 = path.into_inner().parse().unwrap_or(0);
    let users = state.users.lock().unwrap();
    let user = match users.iter().find(|u| u.id == user_id) {
        Some(u) => u,
        None => return HttpResponse::NotFound().json(json!({ "message": "user not found" })),
    };
    let total_amount: u64 = user.bookings.iter().map(|b| b.totalCost).sum();
    HttpResponse::Ok().json(json!({
        "userId": user_id,
        "username": user.username,
        "totalBookings": user.bookings.len(),
        "totalAmountSpent": total_amount
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        global_id: Mutex::new(1),
        global_booking_id: Mutex::new(101),
        users: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_e, _req| {
                actix_web::error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest().json(json!({ "message": "invalid data" })),
                )
                .into()
            }))
            .route("/signup", web::post().to(signup))
            .route("/users", web::get().to(list_users))
            .route("/bookings/{userId}", web::post().to(create_booking))
            .route("/bookings/{userId}", web::get().to(get_user_bookings))
            .route(
                "/bookings/{userId}/{bookingId}/status",
                web::put().to(update_status),
            )
            .route(
                "/bookings/{userId}/{bookingId}",
                web::get().to(get_specific_booking),
            )
            .route(
                "/bookings/{userId}/{bookingId}",
                web::put().to(update_booking),
            )
            .route(
                "/bookings/{userId}/{bookingId}",
                web::delete().to(delete_booking),
            )
            .route("/summary/{userId}", web::get().to(summary))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

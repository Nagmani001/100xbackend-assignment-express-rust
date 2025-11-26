use actix_web::{App, HttpResponse, HttpServer, Responder, Result, web};
use serde::{Deserialize, Serialize, de};
use std::sync::Mutex;

#[derive(Deserialize, Debug)]
struct User_For_Signup {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct SignupResponse {
    message: String,
    userId: u64,
}
async fn signup(
    body: web::Json<User_For_Signup>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let mut data = state.users.lock().unwrap();
    let mut globalId = state.globalId.lock().unwrap();

    let user = User {
        id: *globalId,
        username: body.username.clone(),
        password: body.password.clone(),
        booking: Vec::new(),
    };

    data.push(user);
    *globalId += 1;

    let originalId = *globalId - 1;
    let response = SignupResponse {
        message: String::from("User created successfully"),
        userId: originalId,
    };

    Ok(web::Json(response))
}

async fn users(state: web::Data<AppState>) -> Result<impl Responder> {
    let users = state.users.lock().unwrap();
    let users = users.clone();
    Ok(web::Json(users))
}

#[derive(Deserialize, Debug)]
struct Booking_Create {
    carName: String,
    days: u64,
    rentPerDay: u64,
}

#[derive(Serialize)]
struct Booking_Response {
    message: String,
    bookingId: u64,
    totalCost: u64,
}

async fn booking(
    path: web::Path<(String)>,
    state: web::Data<AppState>,
    body: web::Json<Booking_Create>,
) -> Result<impl Responder> {
    let path = path.into_inner();
    let userId: u64 = path.parse().unwrap();
    let mut users = state.users.lock().unwrap();
    let mut globalBookingId = state.globalBookingId.lock().unwrap();
    let booking = Booking {
        carName: body.carName.clone(),
        days: body.days,
        rentPerDay: body.rentPerDay,
        status: String::from("booked"),
        bookingId: *globalBookingId,
        totalCost: body.rentPerDay * body.days,
    };

    for mut val in users.iter_mut() {
        if val.id == userId {
            val.booking.push(booking);
            break;
        }
    }
    *globalBookingId += 1;

    let response = Booking_Response {
        message: String::from("booking complete"),
        bookingId: *globalBookingId - 1,
        totalCost: body.rentPerDay * body.days,
    };
    println!("{:?}", users);

    Ok(web::Json(response))
}

async fn booking_get(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let path = path.into_inner();
    let user_id: u64 = path.parse().unwrap();
    let users = state.users.lock().unwrap();
    let mut final_booking: Vec<Booking> = vec![];
    for val in users.iter() {
        if val.id == user_id {
            let booking = val.booking.clone();
            final_booking = booking;
            break;
        }
    }
    Ok(web::Json(final_booking))
}

#[derive(Serialize)]
struct Message {
    message: String,
}
async fn get_booking(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let path = path.into_inner();
    let user_id: u64 = path.0.parse().unwrap();
    let booking_id: u64 = path.1.parse().unwrap();
    let users = state.users.lock().unwrap();

    let mut final_booking: Option<Booking> = None;

    let mut is_initialized = false;
    for val in users.iter() {
        if val.id == user_id {
            let bookings = val.booking.clone();
            for booking in bookings {
                if booking.bookingId == booking_id {
                    final_booking = Some(booking);
                    is_initialized = true;
                    break;
                }
            }
        }
    }

    if let Some(booking) = final_booking {
        return Ok(web::Json(booking));
    }

    let message = Message {
        message: String::from("asdf"),
    };

    //WARNING: fix this
    Err(actix_web::error::ErrorNotFound(404))
}

#[derive(Debug, Deserialize)]
struct Update_Body {
    car_name: Option<String>,
    days: Option<u64>,
    rent_per_day: Option<u64>,
}
async fn update_booking(
    path: web::Path<(String, String)>,
    body: web::Json<Update_Body>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let path = path.into_inner();
    let user_id: u64 = path.0.parse().unwrap();
    let booking_id: u64 = path.1.parse().unwrap();
    let mut users = state.users.lock().unwrap();

    let mut booking_to_return: Option<Booking> = None;

    for val in users.iter_mut() {
        if val.id == user_id {
            for booking in val.booking.iter_mut() {
                if booking.bookingId == booking_id {
                    if let Some(days) = body.days {
                        booking.days = days;
                        booking.totalCost = days * booking.rentPerDay;
                    }
                    if let Some(rent_per_day) = body.rent_per_day {
                        booking.rentPerDay = rent_per_day;
                        booking.totalCost = rent_per_day * booking.days;
                    }
                    if let Some(car_name) = body.car_name.clone() {
                        booking.carName = car_name;
                    }

                    booking_to_return = Some(booking.clone());
                }
            }
        }
    }
    if let Some(booking) = booking_to_return {
        Ok(web::Json(booking))
    } else {
        Err(actix_web::error::ErrorNotFound(404))
    }
}

#[derive(Deserialize, Debug)]
struct Update_Booking_Status {
    status: String,
}
#[derive(Serialize)]
struct update_bookking_status_response {
    message: String,
}

async fn update_booking_status(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
    body: web::Json<Update_Booking_Status>,
) -> Result<impl Responder> {
    let path = path.into_inner();
    let user_id: u64 = path.0.parse().unwrap();
    let booking_id: u64 = path.1.parse().unwrap();
    let mut users = state.users.lock().unwrap();

    for val in users.iter_mut() {
        if val.id == user_id {
            for booking in val.booking.iter_mut() {
                if booking.bookingId == booking_id {
                    booking.status = body.status.clone();
                }
            }
        }
    }
    Ok(web::Json(update_bookking_status_response {
        message: String::from("Status updated successfully"),
    }))
}

#[derive(Serialize)]
struct delete_booking_response {
    message: String,
}

async fn delete_booking(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let path = path.into_inner();
    let user_id: u64 = path.0.parse().unwrap();
    let booking_id: u64 = path.1.parse().unwrap();
    let mut users = state.users.lock().unwrap();

    let mut index_to_remove: Option<usize> = None;
    for val in users.iter_mut() {
        if val.id == user_id {
            for (index, booking) in val.booking.iter_mut().enumerate() {
                if booking.bookingId == booking_id {
                    index_to_remove = Some(index)
                }
            }
            if let Some(indexInside) = index_to_remove {
                val.booking.remove(indexInside);
            }
        }
    }
    Ok(web::Json(delete_booking_response {
        message: String::from("Booking deleted successfully"),
    }))
}

#[derive(Serialize)]
struct summary_response {
    userId: u64,
    username: String,
    totalBookings: usize,
    totalAmountSpent: u64,
}

async fn get_summary(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let path = path.into_inner();
    let user_id: u64 = path.parse().unwrap();
    let users = state.users.lock().unwrap();
    let mut total_booking = 0;
    let mut totalAmountSpent = 0;
    let mut user_name = String::new();

    for val in users.iter() {
        if val.id == user_id {
            total_booking = val.booking.len();
            user_name = val.username.clone();
            for booking in val.booking.iter() {
                totalAmountSpent += booking.totalCost;
            }
            break;
        }
    }
    Ok(web::Json(summary_response {
        userId: user_id,
        username: user_name,
        totalBookings: total_booking,
        totalAmountSpent: totalAmountSpent,
    }))
}

#[derive(Debug, Clone, Serialize)]
struct Booking {
    bookingId: u64,
    carName: String,
    days: u64,
    rentPerDay: u64,
    status: String,
    totalCost: u64,
}
#[derive(Debug, Clone, Serialize)]
struct User {
    id: u64,
    username: String,
    password: String,
    booking: Vec<Booking>,
}

#[derive(Debug)]
struct AppState {
    globalId: Mutex<u64>,
    globalBookingId: Mutex<u64>,
    users: Mutex<Vec<User>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        globalId: Mutex::new(1),
        globalBookingId: Mutex::new(101),
        users: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/signup", web::post().to(signup))
            .route("/bookings/{userId}", web::post().to(booking))
            .route("/users", web::get().to(users))
            .route("/bookings/{userId}", web::get().to(booking_get))
            .route("/bookings/{userId}/{bookingId}", web::get().to(get_booking))
            .route(
                "/bookings/{userId}/{bookingId}",
                web::put().to(update_booking),
            )
            .route(
                "/bookings/{userId}/{bookingId}/status",
                web::put().to(update_booking_status),
            )
            .route(
                "/bookings/{userId}/{bookingId}",
                web::delete().to(delete_booking),
            )
            .route("/summary/{userId}", web::get().to(get_summary))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

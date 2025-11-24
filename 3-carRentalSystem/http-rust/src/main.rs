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
    println!("data before = {:?}", data);
    println!("globalId before = {:?}", globalId);

    let user = User {
        id: Mutex::new(*globalId),
        username: Mutex::new(body.username.clone()),
        password: Mutex::new(body.password.clone()),
        booking: Mutex::new(Vec::new()),
    };

    data.push(user);
    *globalId += 1;

    let originalId = *globalId - 1;
    let response = SignupResponse {
        message: String::from("User created successfully"),
        userId: originalId,
    };

    println!("data after = {:?}", data);
    println!("globalId after = {:?}", globalId);
    Ok(web::Json(response))
}

async fn users(state: web::Data<AppState>) -> Result<impl Responder> {
    let data = state.users.lock().unwrap();

    println!("In Memory State = {:?}", state);

    /*
       let mut vec = Vec::new();
    for val in <Vec<User> as Clone>::clone(&data).into_iter() {
        vec.push(val);
    }
      */

    Ok(HttpResponse::Ok().body("hi"))
}

#[derive(Deserialize, Debug)]
struct Booking_Create {
    carName: String,
    days: u64,
    rentPerDay: u64,
}

async fn booking(
    path: web::Path<(String)>,
    state: web::Data<AppState>,
    body: web::Json<Booking_Create>,
) -> impl Responder {
    let path = path.into_inner();
    let path: i32 = path.parse().unwrap();
    println!("path parameter userId = {}", path);
    println!("Request body = {:?}", body);
    println!("In Memory State = {:?}", state);
    HttpResponse::Ok().body("hi")
}

async fn booking_get(path: web::Path<(String)>, state: web::Data<AppState>) -> impl Responder {
    let path = path.into_inner();
    let path: i32 = path.parse().unwrap();

    println!("path parameter userId = {}", path);
    println!("In Memory State = {:?}", state);
    HttpResponse::Ok().body("hi")
}

async fn get_booking(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> impl Responder {
    let path = path.into_inner();
    let userId: i32 = path.0.parse().unwrap();
    let bookingId: i32 = path.1.parse().unwrap();
    println!("{userId}");
    println!("{bookingId}");
    println!("In Memory State = {:?}", state);
    HttpResponse::Ok().body("hi")
}

#[derive(Debug, Deserialize)]
struct Update_Body {
    carName: Option<String>,
    days: Option<i32>,
    rentPerDay: Option<i32>,
}
async fn update_booking(
    path: web::Path<(String, String)>,
    body: web::Json<Update_Body>,
    state: web::Data<AppState>,
) -> impl Responder {
    let path = path.into_inner();
    let userId: i32 = path.0.parse().unwrap();
    let bookingId: i32 = path.1.parse().unwrap();
    println!("userId = {userId}");
    println!("BookingId = {bookingId}");
    println!("Request body = {:?}", body);
    println!("In Memory State = {:?}", state);
    HttpResponse::Ok().body("hi")
}

async fn update_booking_status(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> impl Responder {
    let path = path.into_inner();
    let userId: i32 = path.0.parse().unwrap();
    let bookingId: i32 = path.1.parse().unwrap();
    println!("path parameter userId {:?}", userId);
    println!("path parameter bookingId {:?}", bookingId);
    println!(" in memory state variable {:?}", state);
    HttpResponse::Ok().body("hi")
}

async fn delete_booking(path: web::Path<(String, String)>) -> impl Responder {
    let path = path.into_inner();
    let userId: i32 = path.0.parse().unwrap();
    let bookingId: i32 = path.1.parse().unwrap();
    println!("path parameter userId {:?}", userId);
    println!("path parameter bookingId {:?}", bookingId);
    HttpResponse::Ok().body("hi")
}

async fn get_summary(path: web::Path<(String)>) -> impl Responder {
    let path = path.into_inner();
    let userId: i32 = path.parse().unwrap();
    println!("{:?}", userId);
    HttpResponse::Ok().body("hi")
}

#[derive(Debug)]
struct Booking {
    bookingId: Mutex<i64>,
    carName: Mutex<String>,
    days: Mutex<u64>,
    rentPerDay: Mutex<u64>,
    status: Mutex<String>,
}
#[derive(Debug)]
struct User {
    id: Mutex<u64>,
    username: Mutex<String>,
    password: Mutex<String>,
    booking: Mutex<Vec<Booking>>,
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

use actix_web::{App, HttpResponse, HttpServer, Responder, cookie::time::Date, web};
use serde::{Deserialize, Serialize};

async fn signup() -> impl Responder {
    HttpResponse::Ok().body("hi")
}

async fn movies() -> impl Responder {
    HttpResponse::Ok().body("hi")
}

#[derive(Deserialize, Serialize, Debug)]
struct Shows {
    show_id: i32,
    time: String,
    price_per_seat: i32,
    available_seat: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct Movies {
    id: i32,
    title: String,
    genre: String,
    duration: i32,
    shows: Vec<Shows>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Booking {
    booking_id: i32,
    movie_id: i32,
    show_id: i32,
    seats: i32,
    total_amount: i32,
    status: String,
    booking_date: String,
}

struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
    bookings: Vec<Booking>,
}

struct AppState {
    users: Vec<User>,
    movies: Vec<Movies>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(signup))
            .route("/", web::get().to(movies))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

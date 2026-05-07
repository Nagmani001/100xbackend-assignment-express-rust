use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Mutex;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Show {
    showId: u32,
    time: String,
    pricePerSeat: u32,
    availableSeats: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Movie {
    id: u32,
    title: String,
    genre: String,
    duration: u32,
    shows: Vec<Show>,
}

#[derive(Clone, Serialize, Debug)]
struct Booking {
    bookingId: u32,
    movieId: u32,
    showId: u32,
    seats: u32,
    totalAmount: u32,
    status: String,
}

#[derive(Clone, Serialize, Debug)]
struct User {
    id: u32,
    username: String,
    email: String,
    password: String,
    bookings: Vec<Booking>,
}

struct AppState {
    user_id: Mutex<u32>,
    booking_id: Mutex<u32>,
    users: Mutex<Vec<User>>,
    movies: Mutex<Vec<Movie>>,
}

fn default_movies() -> Vec<Movie> {
    vec![
        Movie {
            id: 1,
            title: "Inception".into(),
            genre: "Sci-Fi".into(),
            duration: 148,
            shows: vec![
                Show { showId: 101, time: "10:00 AM".into(), pricePerSeat: 200, availableSeats: 50 },
                Show { showId: 102, time: "2:00 PM".into(), pricePerSeat: 250, availableSeats: 50 },
                Show { showId: 103, time: "6:00 PM".into(), pricePerSeat: 300, availableSeats: 50 },
            ],
        },
        Movie {
            id: 2,
            title: "The Dark Knight".into(),
            genre: "Action".into(),
            duration: 152,
            shows: vec![
                Show { showId: 201, time: "11:00 AM".into(), pricePerSeat: 200, availableSeats: 50 },
                Show { showId: 202, time: "3:00 PM".into(), pricePerSeat: 250, availableSeats: 50 },
                Show { showId: 203, time: "7:00 PM".into(), pricePerSeat: 300, availableSeats: 50 },
            ],
        },
        Movie {
            id: 3,
            title: "Interstellar".into(),
            genre: "Sci-Fi".into(),
            duration: 169,
            shows: vec![
                Show { showId: 301, time: "12:00 PM".into(), pricePerSeat: 250, availableSeats: 50 },
                Show { showId: 302, time: "5:00 PM".into(), pricePerSeat: 300, availableSeats: 50 },
            ],
        },
    ]
}

fn valid_email(e: &str) -> bool {
    let parts: Vec<&str> = e.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() {
        return false;
    }
    parts[1].contains('.') && !parts[1].starts_with('.') && !parts[1].ends_with('.')
}

async fn signup(state: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
    let username = body.get("username").and_then(|v| v.as_str());
    let email = body.get("email").and_then(|v| v.as_str());
    let password = body.get("password").and_then(|v| v.as_str());
    let (username, email, password) = match (username, email, password) {
        (Some(u), Some(e), Some(p)) if valid_email(e) => (u.to_string(), e.to_string(), p.to_string()),
        _ => return HttpResponse::BadRequest().json(json!({ "message": "invalid input" })),
    };
    let mut users = state.users.lock().unwrap();
    if users.iter().any(|u| u.email == email) {
        return HttpResponse::Unauthorized().json(json!({ "message": "user already exists" }));
    }
    let mut uid = state.user_id.lock().unwrap();
    let id = *uid;
    *uid += 1;
    users.push(User {
        id,
        username,
        email,
        password,
        bookings: Vec::new(),
    });
    HttpResponse::Created().json(json!({ "message": "User created successfully", "userId": id }))
}

async fn list_movies(state: web::Data<AppState>) -> impl Responder {
    let movies = state.movies.lock().unwrap();
    HttpResponse::Ok().json(json!({ "movies": movies.clone() }))
}

async fn movie_by_id(path: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let movies = state.movies.lock().unwrap();
    match movies.iter().find(|m| m.id == id) {
        Some(m) => HttpResponse::Ok().json(m),
        None => HttpResponse::NotFound().json(json!({ "message": "Movie not found" })),
    }
}

async fn movie_shows(path: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let movies = state.movies.lock().unwrap();
    match movies.iter().find(|m| m.id == id) {
        Some(m) => HttpResponse::Ok().json(json!({ "shows": m.shows })),
        None => HttpResponse::NotFound().json(json!({ "message": "Movie not found" })),
    }
}

#[derive(Deserialize)]
struct BookingInput {
    movieId: u32,
    showId: u32,
    seats: u32,
}

async fn create_booking(
    path: web::Path<u32>,
    state: web::Data<AppState>,
    body: web::Json<BookingInput>,
) -> impl Responder {
    let user_id = path.into_inner();
    let mut users = state.users.lock().unwrap();
    let user = match users.iter_mut().find(|u| u.id == user_id) {
        Some(u) => u,
        None => return HttpResponse::NotFound().json(json!({ "message": "user not found" })),
    };
    let mut movies = state.movies.lock().unwrap();
    let movie = match movies.iter_mut().find(|m| m.id == body.movieId) {
        Some(m) => m,
        None => return HttpResponse::NotFound().json(json!({ "message": "Movie not found" })),
    };
    let movie_title = movie.title.clone();
    let show = match movie.shows.iter_mut().find(|s| s.showId == body.showId) {
        Some(s) => s,
        None => return HttpResponse::NotFound().json(json!({ "message": "Show not found" })),
    };
    if show.availableSeats < body.seats {
        return HttpResponse::BadRequest().json(json!({ "message": "Not enough seats available" }));
    }
    show.availableSeats -= body.seats;
    let total_amount = show.pricePerSeat * body.seats;
    let show_time = show.time.clone();
    let mut bid = state.booking_id.lock().unwrap();
    let booking_id = *bid;
    *bid += 1;
    user.bookings.push(Booking {
        bookingId: booking_id,
        movieId: body.movieId,
        showId: body.showId,
        seats: body.seats,
        totalAmount: total_amount,
        status: "confirmed".into(),
    });
    HttpResponse::Created().json(json!({
        "message": "Booking successful",
        "bookingId": booking_id,
        "movieTitle": movie_title,
        "showTime": show_time,
        "seats": body.seats,
        "totalAmount": total_amount
    }))
}

async fn get_user_bookings(path: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let users = state.users.lock().unwrap();
    match users.iter().find(|u| u.id == user_id) {
        Some(u) => HttpResponse::Ok().json(json!({ "bookings": u.bookings.clone() })),
        None => HttpResponse::NotFound().json(json!({ "message": "user not found" })),
    }
}

async fn get_specific_booking(
    path: web::Path<(u32, u32)>,
    state: web::Data<AppState>,
) -> impl Responder {
    let (uid, bid) = path.into_inner();
    let users = state.users.lock().unwrap();
    let booking = users
        .iter()
        .find(|u| u.id == uid)
        .and_then(|u| u.bookings.iter().find(|b| b.bookingId == bid));
    match booking {
        Some(b) => HttpResponse::Ok().json(b),
        None => HttpResponse::NotFound().json(json!({ "message": "Booking not found" })),
    }
}

#[derive(Deserialize)]
struct UpdateBooking {
    seats: u32,
}

async fn update_booking(
    path: web::Path<(u32, u32)>,
    state: web::Data<AppState>,
    body: web::Json<UpdateBooking>,
) -> impl Responder {
    let (uid, bid) = path.into_inner();
    let mut users = state.users.lock().unwrap();
    let mut movies = state.movies.lock().unwrap();
    let user = match users.iter_mut().find(|u| u.id == uid) {
        Some(u) => u,
        None => return HttpResponse::NotFound().json(json!({ "message": "user not found" })),
    };
    let booking = match user.bookings.iter_mut().find(|b| b.bookingId == bid) {
        Some(b) => b,
        None => return HttpResponse::NotFound().json(json!({ "message": "Booking not found" })),
    };
    let movie = match movies.iter_mut().find(|m| m.id == booking.movieId) {
        Some(m) => m,
        None => return HttpResponse::NotFound().json(json!({ "message": "Movie not found" })),
    };
    let show = match movie.shows.iter_mut().find(|s| s.showId == booking.showId) {
        Some(s) => s,
        None => return HttpResponse::NotFound().json(json!({ "message": "Show not found" })),
    };
    if show.availableSeats < body.seats {
        return HttpResponse::BadRequest().json(json!({ "message": "not enough seats" }));
    }
    show.availableSeats -= body.seats;
    booking.seats += body.seats;
    booking.totalAmount = booking.seats * show.pricePerSeat;
    HttpResponse::Ok().json(json!({
        "message": "Booking updated successfully",
        "bookingId": booking.bookingId,
        "seats": booking.seats,
        "totalAmount": booking.totalAmount
    }))
}

async fn delete_booking(
    path: web::Path<(u32, u32)>,
    state: web::Data<AppState>,
) -> impl Responder {
    let (uid, bid) = path.into_inner();
    let mut users = state.users.lock().unwrap();
    let booking = users
        .iter_mut()
        .find(|u| u.id == uid)
        .and_then(|u| u.bookings.iter_mut().find(|b| b.bookingId == bid));
    match booking {
        Some(b) => {
            b.status = "cancelled".into();
            HttpResponse::Ok().json(json!({ "message": "Booking cancelled successfully" }))
        }
        None => HttpResponse::NotFound().json(json!({ "message": "Booking not found" })),
    }
}

async fn summary(path: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let uid = path.into_inner();
    let users = state.users.lock().unwrap();
    let user = match users.iter().find(|u| u.id == uid) {
        Some(u) => u,
        None => return HttpResponse::NotFound().json(json!({ "message": "user not found" })),
    };
    let mut total_amount = 0u32;
    let mut confirmed = 0u32;
    let mut cancelled = 0u32;
    let mut total_seats = 0u32;
    for b in &user.bookings {
        total_amount += b.totalAmount;
        total_seats += b.seats;
        if b.status == "confirmed" {
            confirmed += 1;
        } else if b.status == "cancelled" {
            cancelled += 1;
        }
    }
    HttpResponse::Ok().json(json!({
        "userId": uid,
        "username": user.username,
        "totalBookings": user.bookings.len(),
        "totalAmountSpent": total_amount,
        "confirmedBookings": confirmed,
        "cancelledBookings": cancelled,
        "totalSeatsBooked": total_seats
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        user_id: Mutex::new(1),
        booking_id: Mutex::new(1001),
        users: Mutex::new(Vec::new()),
        movies: Mutex::new(default_movies()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(web::JsonConfig::default().error_handler(|_e, _req| {
                actix_web::error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest().json(json!({ "message": "invalid input" })),
                )
                .into()
            }))
            .route("/signup", web::post().to(signup))
            .route("/movies", web::get().to(list_movies))
            .route("/movies/{id}", web::get().to(movie_by_id))
            .route("/movies/{id}/shows", web::get().to(movie_shows))
            .route("/bookings/{userId}", web::post().to(create_booking))
            .route("/bookings/{userId}", web::get().to(get_user_bookings))
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

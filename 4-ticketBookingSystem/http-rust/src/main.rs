use actix_web::{App, HttpResponse, HttpServer, Responder, Result, cookie::time::Date, web};
use chrono::prelude::*;
use serde::{Deserialize, Serialize, de};
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    sync::Mutex,
};

#[derive(Deserialize, Serialize, Debug)]
struct Shows {
    showId: i32,
    time: String,
    pricePerSeat: i32,
    availableSeats: i32,
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
    bookingId: i32,
    movieId: i32,
    showId: i32,
    seats: i32,
    totalAmount: i32,
    status: String,
    bookingDate: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
    bookings: Vec<Booking>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    users: Vec<User>,
    movies: Vec<Movies>,
}

#[derive(Deserialize, Debug)]
struct signup_input {
    email: String,
    password: String,
    username: String,
}

#[derive(Serialize)]
struct signup_response {
    message: String,
    userId: i32,
}

async fn signup(
    state: web::Data<AppState>,
    body: web::Json<signup_input>,
) -> Result<impl Responder> {
    let fild_data = fs::read_to_string("data.txt").unwrap();
    let mut user_id = state.user_id.lock().unwrap();
    let mut parsed_data: Data = serde_json::from_str(&fild_data).unwrap();

    parsed_data.users.push(User {
        id: *user_id,
        username: body.username.clone(),
        password: body.password.clone(),
        email: body.email.clone(),
        bookings: Vec::new(),
    });

    let response = signup_response {
        message: String::from("User created successfully"),
        userId: *user_id,
    };
    *user_id += 1;

    let json = serde_json::to_string(&parsed_data).unwrap();

    let file = File::create("data.txt").unwrap();
    let mut buff_writer = BufWriter::new(file);

    buff_writer.write_all(json.as_bytes()).unwrap();
    Ok(web::Json(response))
}

async fn movies(state: web::Data<AppState>) -> Result<impl Responder> {
    let fild_data = fs::read_to_string("data.txt").unwrap();
    let mut parsed_data: Data = serde_json::from_str(&fild_data).unwrap();

    Ok(web::Json(parsed_data.movies))
}

async fn movie_by_id(path: web::Path<i32>) -> Result<impl Responder> {
    let movie_id = path.into_inner();

    let fild_data = fs::read_to_string("data.txt").unwrap();
    let parsed_data: Data = serde_json::from_str(&fild_data).unwrap();
    let mut movie_to_send: Option<Movies> = None;

    for val in parsed_data.movies {
        if val.id == movie_id {
            movie_to_send = Some(val);
        }
    }
    if let Some(movie) = movie_to_send {
        return Ok(web::Json(movie));
    } else {
        Err(actix_web::error::ErrorNotFound(404))
    }
}

async fn shows_of_movie_id(path: web::Path<i32>) -> Result<impl Responder> {
    let movie_id = path.into_inner();

    let fild_data = fs::read_to_string("data.txt").unwrap();
    let parsed_data: Data = serde_json::from_str(&fild_data).unwrap();
    let mut movie_to_send: Option<Movies> = None;

    for val in parsed_data.movies {
        if val.id == movie_id {
            movie_to_send = Some(val);
        }
    }
    if let Some(movie) = movie_to_send {
        return Ok(web::Json(movie.shows));
    } else {
        Err(actix_web::error::ErrorNotFound(404))
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    message: String,
}

#[derive(Deserialize, Debug)]
struct add_booking_input {
    movie_id: i32,
    show_id: i32,
    seats: i32,
}

#[derive(Serialize)]
struct add_booking_output {
    message: String,
    booking_id: i32,
    seats: i32,
    total_amount: i32,
}

async fn add_booking(
    body: web::Json<add_booking_input>,
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let user_id = path.into_inner();
    let fild_data = fs::read_to_string("data.txt").unwrap();
    let mut parsed_data: Data = serde_json::from_str(&fild_data).unwrap();
    let mut global_booking_id = state.booking_id.lock().unwrap();
    let mut amout = 0;

    for val in parsed_data.movies.iter() {
        for show in val.shows.iter() {
            if show.availableSeats < body.seats {
                return Err(actix_web::error::ErrorNotFound(404));
            }
        }
    }

    for val in parsed_data.movies.iter_mut() {
        for show in val.shows.iter_mut() {
            if show.showId == body.show_id {
                let altogether = show.pricePerSeat * body.seats;
                amout = altogether;
                show.availableSeats -= body.seats;
            }
        }
    }

    for val in parsed_data.users.iter_mut() {
        if val.id == user_id {
            val.bookings.push(Booking {
                bookingId: *global_booking_id,
                movieId: body.movie_id,
                seats: body.seats,
                showId: body.show_id,
                totalAmount: amout,
                status: String::from("confirmed"),
                bookingDate: Utc::now().to_string(),
            });
        }
    }

    let json = serde_json::to_string(&parsed_data).unwrap();

    let file = File::create("data.txt").unwrap();
    let mut buff_writer = BufWriter::new(file);

    buff_writer.write_all(json.as_bytes()).unwrap();
    let response = add_booking_output {
        message: String::from("Booking updated successfully"),
        booking_id: *global_booking_id,
        seats: body.seats,
        total_amount: amout,
    };
    *global_booking_id += 1;

    Ok(web::Json(response))
}

async fn get_bookingss_of_user(path: web::Path<i32>) -> Result<impl Responder> {
    let user_id = path.into_inner();
    let fild_data = fs::read_to_string("data.txt").unwrap();
    let parsed_data: Data = serde_json::from_str(&fild_data).unwrap();
    let mut booking_to_send: Option<Vec<Booking>> = None;

    for val in parsed_data.users {
        if val.id == user_id {
            booking_to_send = Some(val.bookings)
        }
    }

    if let Some(bookings) = booking_to_send {
        Ok(web::Json(bookings))
    } else {
        Err(actix_web::error::ErrorNotFound(404))
    }
}

async fn get_specific_booking_for_user(path: web::Path<(i32, i32)>) -> Result<impl Responder> {
    let path = path.into_inner();
    let user_id = path.0;
    let booking_id = path.1;
    let fild_data = fs::read_to_string("data.txt").unwrap();
    let parsed_data: Data = serde_json::from_str(&fild_data).unwrap();
    let mut booking_to_send: Option<Booking> = None;

    for val in parsed_data.users {
        if val.id == user_id {
            for val in val.bookings {
                if val.bookingId == booking_id {
                    booking_to_send = Some(val);
                    break;
                }
            }
            break;
        }
    }

    if let Some(bookings) = booking_to_send {
        Ok(web::Json(bookings))
    } else {
        Err(actix_web::error::ErrorNotFound(404))
    }
}

#[derive(Deserialize, Debug)]
struct update_bookings_for_user {
    seats: i32,
}

#[derive(Serialize, Debug, Clone)]
struct update_bookings_for_user_response {
    message: String,
    bookingId: i32,
    seats: i32,
    totalAmount: i32,
}

/*
 //WARNING: my code

async fn put_bookingss_for_user(
    body: web::Json<update_bookings_for_user>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder> {
    let path = path.into_inner();
    let user_id = path.0;
    let booking_id = path.1;
    let fild_data = fs::read_to_string("data.txt").unwrap();
    let mut parsed_data: Data = serde_json::from_str(&fild_data).unwrap();
    let mut movie_id: Option<i32> = None;
    let mut show_id: Option<i32> = None;

    let mut found_show: Option<&Shows> = None;
    for val in parsed_data.users.iter() {
        if val.id == user_id {
            for bookings in val.bookings.iter() {
                if bookings.bookingId == booking_id {
                    movie_id = Some(bookings.movieId);
                    show_id = Some(bookings.showId);
                }
            }
        }
    }

    if let Some(movieId) = movie_id {
        if let Some(show_id) = show_id {
            for val in parsed_data.movies.iter() {
                if val.id == movieId {
                    for show in val.shows.iter() {
                        if show.showId == show_id {
                            found_show = Some(show)
                        }
                    }
                }
            }
        } else {
            println!("error finding the show id");
            return Err(actix_web::error::ErrorNotFound(404));
        }

        if let Some(show) = found_show {
            if show.availableSeats >= body.seats {
                show.availableSeats -= body.seats;

                for val in parsed_data.users.iter() {
                    if val.id == user_id {
                        for bookings in val.bookings.iter_mut() {
                            if bookings.bookingId == booking_id {
                                bookings.seats += body.seats;
                                bookings.totalAmount += body.seats * show.pricePerSeat;

                                let json = serde_json::to_string(&parsed_data).unwrap();

                                let file = File::create("data.txt").unwrap();
                                let mut buff_writer = BufWriter::new(file);

                                buff_writer.write_all(json.as_bytes()).unwrap();
                                let response = update_bookings_for_user_response {
                                    message: String::from("Booking updated successfully"),
                                    bookingId: booking_id,
                                    seats: body.seats,
                                    totalAmount: bookings.totalAmount,
                                };
                                return Ok(web::Json(response.clone()));
                            }
                        }
                    }
                }
            }
            println!("somehow the response wasn't returned");

            return Err(actix_web::error::ErrorNotFound(404));
        } else {
            println!("found_shows error");
            return Err(actix_web::error::ErrorNotFound(404));
        }
    } else {
        println!("movie id was not found ");
        return Err(actix_web::error::ErrorNotFound(404));
    }
}
 */

/*
// WARNING: gpt code

async fn put_bookingss_for_user(
    body: web::Json<update_bookings_for_user>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder> {
    let (user_id, booking_id) = path.into_inner();

    // Read file and parse to struct
    let file_data = fs::read_to_string("data.txt").unwrap();
    let mut parsed_data: Data = serde_json::from_str(&file_data).unwrap();

    // --- find user ---
    let user = parsed_data
        .users
        .iter_mut()
        .find(|u| u.id == user_id)
        .ok_or(actix_web::error::ErrorNotFound("User not found"))?;

    // --- find booking ---
    let booking = user
        .bookings
        .iter_mut()
        .find(|b| b.bookingId == booking_id)
        .ok_or(actix_web::error::ErrorNotFound("Booking not found"))?;

    // --- find movie ---
    let movie = parsed_data
        .movies
        .iter_mut()
        .find(|m| m.id == booking.movieId)
        .ok_or(actix_web::error::ErrorNotFound("Movie not found"))?;

    // --- find show ---
    let show = movie
        .shows
        .iter_mut()
        .find(|s| s.showId == booking.showId)
        .ok_or(actix_web::error::ErrorNotFound("Show not found"))?;

    // --- Business Logic ---
    if show.availableSeats < body.seats {
        return Err(actix_web::error::ErrorNotFound("Not enough seats"));
    }

    // deduct seats from show
    show.availableSeats -= body.seats;

    // update booking

    // ---- update booking and show ----
    booking.seats += body.seats;
    booking.totalAmount += body.seats * show.pricePerSeat;

    // store values BEFORE borrow ends
    let total_amount = booking.totalAmount;
    let resp_seats = body.seats;

    // ⬇ End the borrow scope explicitly
    drop(show);
    drop(booking);
    drop(user);

    // ---- now we can serialize ----
    let json = serde_json::to_string(&parsed_data).unwrap();
    let file = File::create("data.txt").unwrap();
    BufWriter::new(file).write_all(json.as_bytes()).unwrap();

    // ---- Response ----
    let response = update_bookings_for_user_response {
        message: "Booking updated successfully".into(),
        bookingId: booking_id,
        seats: resp_seats,
        totalAmount: total_amount,
    };

    Ok(web::Json(response))
}

*/

async fn delete_bookingss_for_user(path: web::Path<(i32, i32)>) -> Result<impl Responder> {
    let (user_id, booking_id) = path.into_inner();

    let file_data = fs::read_to_string("data.txt").unwrap();
    let mut parsed_data: Data = serde_json::from_str(&file_data).unwrap();
    for val in parsed_data.users.iter_mut() {
        if user_id == val.id {
            for booking in val.bookings.iter_mut() {
                if booking.bookingId == booking_id {
                    booking.status = String::from("cancelled");
                }
            }
        }
    }

    let json = serde_json::to_string(&parsed_data).unwrap();

    let file = File::create("data.txt").unwrap();
    let mut buff_writer = BufWriter::new(file);

    buff_writer.write_all(json.as_bytes()).unwrap();

    Ok(web::Json(Message {
        message: String::from("Booking cancelled successfully"),
    }))
}

#[derive(Serialize)]
struct summary_response {
    userId: i32,
    username: String,
    totalBookings: i32,
    totalAmountSpent: i32,
    confirmedBookings: i32,
    cancelledBookings: i32,
    totalSeatsBooked: i32,
}

async fn summary(path: web::Path<i32>) -> impl Responder {
    let user_id = path.into_inner();

    let file_data = fs::read_to_string("data.txt").unwrap();
    let mut parsed_data: Data = serde_json::from_str(&file_data).unwrap();
    let mut user_name: Option<String> = None;
    let mut total_bookings = 0;
    let mut total_amount_spent = 0;
    let mut confirmed_bookings = 0;
    let mut cancelled_bookings = 0;
    let mut total_seats_booked = 0;

    for user in parsed_data.users {
        if user.id == user_id {
            user_name = Some(user.username);
            for booking in user.bookings {
                total_bookings += 1;
                total_amount_spent += booking.totalAmount;
                if booking.status == String::from("confirmed") {
                    confirmed_bookings += 1;
                }

                if booking.status == String::from("calcelled") {
                    cancelled_bookings += 1;
                }
                total_seats_booked += booking.seats;
            }
        }
    }

    if let Some(username) = user_name {
        let response = summary_response {
            userId: user_id,
            username: username,
            totalBookings: total_bookings,
            totalAmountSpent: total_amount_spent,
            confirmedBookings: confirmed_bookings,
            cancelledBookings: cancelled_bookings,
            totalSeatsBooked: total_seats_booked,
        };
        Ok(web::Json(response))
    } else {
        Err(actix_web::error::ErrorNotFound(404))
    }
}

struct AppState {
    user_id: Mutex<i32>,
    booking_id: Mutex<i32>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        user_id: Mutex::new(1),
        booking_id: Mutex::new(1001),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/signup", web::post().to(signup))
            .route("/movies", web::get().to(movies))
            .route("/movies/{movie_id}", web::get().to(movie_by_id))
            .route("/movies/{movid_id}/shows", web::get().to(shows_of_movie_id))
            .route("/bookings/{user_id}", web::post().to(add_booking))
            .route("/bookings/{user_id}", web::get().to(get_bookingss_of_user))
            .route(
                "/bookings/{user_id}/{booking_id}",
                web::get().to(get_specific_booking_for_user),
            )
            /*
            .route(
                "/bookings/{user_id}/{booking_id}",
                web::put().to(put_bookingss_for_user),
            )
             * */
            .route(
                "/bookings/{user_id}/{booking_id}",
                web::delete().to(delete_bookingss_for_user),
            )
            .route("/summary/{user_id}", web::get().to(summary))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

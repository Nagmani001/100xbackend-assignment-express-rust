use std::fs::File;
use std::io::Write;
use std::{fs, sync::Mutex};

use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    web::{self, Data},
};
use serde::Deserialize;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to my first Express or Actix server!")
}

#[derive(Deserialize, Debug)]
struct Name {
    name: String,
}

async fn greet(name: web::Query<Name>) -> impl Responder {
    let message = format!("Hello {}, nice to meet you!", name.name);
    HttpResponse::Ok().body(message)
}

#[derive(Deserialize, Debug)]
struct Message {
    message: String,
}

async fn write_to_file(message: web::Query<Message>) -> impl Responder {
    //TODO: why do i have to clone here , understanding the error which came without clone()
    fs::write("notes.txt", message.message.clone()).unwrap();
    HttpResponse::Ok().body("wrote to file")
}

async fn append_to_file(message: web::Query<Message>) -> impl Responder {
    let mut f = File::options().append(true).open("notes.txt").unwrap();
    let actual_message = message.message.clone();
    writeln!(&mut f, "{}", actual_message).unwrap();
    HttpResponse::Ok().body("appended to file")
}

async fn read() -> impl Responder {
    let data = fs::read_to_string("notes.txt");
    match data {
        Ok(val) => HttpResponse::Ok().body(val),
        Err(_) => HttpResponse::NotFound().body("Error while reading file "),
    }
}

async fn clear() -> impl Responder {
    fs::write("notes.txt", "").unwrap();
    HttpResponse::Ok().body("cleared")
}

#[derive(Deserialize, Debug)]
struct User {
    name: Mutex<String>,
    age: Mutex<u32>,
}

async fn add_users(user: web::Query<User>, data: web::Data<Vec<User>>) -> impl Responder {
    HttpResponse::Ok().body("User added successfully!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut users: Vec<User> = vec![];
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(users.clone()))
            .route("/", web::get().to(hello))
            .route("/greet", web::get().to(greet))
            .route("/write", web::get().to(write_to_file))
            .route("/append", web::get().to(append_to_file))
            .route("/read", web::get().to(read))
            .route("/clear", web::get().to(clear))
            .route("/add-users", web::get().to(add_users))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

use std::fs::File;
use std::io::Write;
use std::{fs, sync::Mutex};

use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    web::{self, Data},
};
use serde::{Deserialize, Serialize};

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
    fs::write("notes.txt", &message.message).unwrap();
    HttpResponse::Ok().body("wrote to file")
}

async fn append_to_file(message: web::Query<Message>) -> impl Responder {
    let mut f = File::options().append(true).open("notes.txt").unwrap();
    let actual_message = &message.message;
    writeln!(&mut f, "{}", actual_message).unwrap();
    HttpResponse::Ok().body("appended to file")
}

async fn read() -> impl Responder {
    let data = fs::read_to_string("notes.txt").unwrap();
    //TODO: even though i am returning the right thing from backend , the test fails
    //but it passes with express , also there could be issue with how i should be wrigint tests
    println!("{}", data);
    HttpResponse::Ok().body(data)
}

async fn clear() -> impl Responder {
    fs::write("notes.txt", "").unwrap();
    HttpResponse::Ok().body("cleared")
}

#[derive(Deserialize, Debug, Clone, Serialize)]
struct User {
    name: String,
    age: u32,
}

#[derive(Debug)]
struct Users {
    user: Mutex<Vec<User>>,
}

async fn add_users(user: web::Query<User>, data: web::Data<AppData>) -> impl Responder {
    let mut users = data.users.user.lock().unwrap();
    let user = User {
        name: user.name.clone(),
        age: user.age,
    };
    users.push(user);

    HttpResponse::Ok().body("User added successfully!")
}

#[derive(Debug)]
struct BlockedUser {
    user: (String, bool),
}

#[derive(Debug)]
struct BlockedUsers {
    users: Mutex<Vec<BlockedUser>>,
}

struct AppData {
    users: Users,
    blocked_users: BlockedUsers,
}

async fn check_users(user: web::Query<User>, data: web::Data<AppData>) -> impl Responder {
    let mut blockedUsers = data.blocked_users.users.lock().unwrap();
    if user.age < 18 {
        blockedUsers.push(BlockedUser {
            user: (user.name.clone(), true),
        });
        HttpResponse::Ok().body("You are blocked as your age is less.")
    } else {
        blockedUsers.push(BlockedUser {
            user: (user.name.clone(), false),
        });
        HttpResponse::Ok().body("Access granted!")
    }
}

#[derive(Deserialize)]
struct CheckUser {
    name: String,
}

async fn is_blocked(user: web::Query<CheckUser>, data: web::Data<AppData>) -> impl Responder {
    let users_to_loop = data.blocked_users.users.lock().unwrap();
    let mut string = String::new();
    for val in users_to_loop.iter() {
        if val.user.0 == user.name {
            if val.user.1 {
                let respond_with = format!("{} is blocked.", val.user.0);
                string = respond_with;
            } else {
                let respond_with = format!("{} is not blocked.", val.user.0);
                string = respond_with;
            }
        }
    }
    HttpResponse::Ok().body(string)
}

async fn users(data: web::Data<AppData>) -> impl Responder {
    let mut mutex = data.users.user.lock().unwrap();
    let user_vec = std::mem::take(&mut *mutex);
    let json = serde_json::to_string(&user_vec).unwrap();

    //TODO: only reponds with the user for the first time, fix this
    HttpResponse::Ok().body(json)
}

async fn clear_data(data: web::Data<AppData>) -> impl Responder {
    let mut mutex = data.users.user.lock().unwrap();

    //WARNING: does this actually empty the users and blocked users ? , how do you test this

    let mut users_data = std::mem::take(&mut *mutex);
    users_data = Vec::new();

    let mut mutex1 = data.blocked_users.users.lock().unwrap();

    let mut blocked_user = std::mem::take(&mut *mutex1);
    blocked_user = Vec::new();

    HttpResponse::Ok().body("All data cleared successfully!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let users_data = Users {
        user: Mutex::new(vec![]),
    };

    let blocked_users_data = BlockedUsers {
        users: Mutex::new(vec![]),
    };
    let app_data = web::Data::new(AppData {
        users: users_data,
        blocked_users: blocked_users_data,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/", web::get().to(hello))
            .route("/greet", web::get().to(greet))
            .route("/write", web::get().to(write_to_file))
            .route("/append", web::get().to(append_to_file))
            .route("/read", web::get().to(read))
            .route("/clear", web::get().to(clear))
            .route("/add-user", web::get().to(add_users))
            .route("/check-user", web::get().to(check_users))
            .route("/is-blocked", web::get().to(is_blocked))
            .route("/users", web::get().to(users))
            .route("/clear-data", web::get().to(clear_data))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

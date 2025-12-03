use actix_web::{App, Error, HttpResponse, HttpServer, Responder, Result, dev::ResourcePath, web};
use serde::{Deserialize, Serialize, de};
use std::sync::Mutex;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to TODO Backend!")
}

#[derive(Deserialize, Debug)]
struct TodoInput {
    task: String,
}

#[derive(Serialize)]
struct Add_todo_resposne {
    message: String,
    todoCount: usize,
}
async fn add_todo(
    state: web::Data<AppState>,
    todo: web::Query<TodoInput>,
) -> Result<impl Responder> {
    let mut todos = state.todos.lock().unwrap();
    let mut globalId = state.globalId.lock().unwrap();
    let todo_tobe_added = Todo {
        id: *globalId,
        task: todo.task.clone(),
        completed: false,
    };

    todos.push(todo_tobe_added);
    *globalId += 1;

    let response = Add_todo_resposne {
        message: String::from("TODO added successfully!"),
        todoCount: todos.len(),
    };

    Ok(web::Json(response))
}

async fn todos(state: web::Data<AppState>) -> impl Responder {
    let todos = state.todos.lock().unwrap();

    // send status 404 when no todo is found
    //    if todos.len() == 0 {}

    // otherwise send a json with all the todos
    HttpResponse::Ok().body("hi")
}

#[derive(Serialize)]
struct Todo_by_id_response {
    error: String,
}

async fn todo_by_id(path: web::Path<String>, state: web::Data<AppState>) -> Result<impl Responder> {
    let path = path.into_inner();
    let id: i32 = path.parse().unwrap();
    let todos = state.todos.lock().unwrap();
    let mut found_todo: Option<&Todo> = None;

    for todo in todos.iter() {
        if todo.id == id {
            found_todo = Some(todo);
            break;
        }
    }
    let response = Todo_by_id_response {
        error: String::from("TODO not found"),
    };

    //TODO: remove this and write the code to return response or with error code and message
    Ok(web::Json(Todo_by_id_response {
        error: String::from("ASdf"),
    }))
    /*
    if let Some(todo) = found_todo {
        Ok(web::Json(todo))
    } else {
    }
      */
}

async fn mark_as_complete(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let todos = state.todos.lock().unwrap();
    let path = path.into_inner();
    let id: i32 = path.parse().unwrap();
    println!("{:?}", todos);
    println!("{:?}", id);

    HttpResponse::Ok().body("hi")
}

async fn delete_todo_by_id(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let todos = state.todos.lock().unwrap();
    let path = path.into_inner();
    let id: i32 = path.parse().unwrap();
    println!("{:?}", todos);
    println!("{:?}", id);

    HttpResponse::Ok().body("hi")
}

#[derive(Debug, Deserialize)]
struct Filter_todo_input {
    status: String,
}

async fn filter_todo(query: web::Query<Filter_todo_input>) -> impl Responder {
    println!("{:?}", query);
    HttpResponse::Ok().body("hi")
}

async fn save_todos() -> impl Responder {
    HttpResponse::Ok().body("hi")
}

async fn load_todos() -> impl Responder {
    HttpResponse::Ok().body("hi")
}

async fn clear_todos() -> impl Responder {
    HttpResponse::Ok().body("hi")
}

async fn stats() -> impl Responder {
    HttpResponse::Ok().body("hi")
}

#[derive(Serialize, Debug, Deserialize)]
struct Todo {
    id: i32,
    task: String,
    completed: bool,
}

#[derive(Serialize, Debug)]
struct AppState {
    globalId: Mutex<i32>,
    todos: Mutex<Vec<Todo>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let appSate = web::Data::new(AppState {
        globalId: Mutex::new(1),
        todos: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(appSate.clone())
            .route("/", web::get().to(hello))
            .route("/add-todo", web::post().to(add_todo))
            .route("/todos", web::get().to(todos))
            .route("/todo/{id}", web::get().to(todo_by_id))
            .route("/todo/{id}/complete", web::put().to(mark_as_complete))
            .route("/todo/{id}", web::delete().to(delete_todo_by_id))
            .route("/todos/filter", web::get().to(filter_todo))
            .route("/save-todos", web::post().to(save_todos))
            .route("/load-todos", web::post().to(load_todos))
            .route("/clear-todos", web::delete().to(clear_todos))
            .route("/stats", web::get().to(stats))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

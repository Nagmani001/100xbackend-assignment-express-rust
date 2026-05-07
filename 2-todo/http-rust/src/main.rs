use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    sync::Mutex,
};

#[derive(Serialize, Debug, Deserialize, Clone)]
struct Todo {
    id: i32,
    task: String,
    completed: bool,
}

#[derive(Serialize, Debug, Deserialize)]
struct AppState {
    global_id: Mutex<i32>,
    todos: Mutex<Vec<Todo>>,
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to TODO Backend!")
}

#[derive(Deserialize, Debug)]
struct TodoInput {
    task: String,
}

async fn add_todo(state: web::Data<AppState>, todo: web::Query<TodoInput>) -> impl Responder {
    let mut todos = state.todos.lock().unwrap();
    let mut global_id = state.global_id.lock().unwrap();
    let new_todo = Todo {
        id: *global_id,
        task: todo.task.clone(),
        completed: false,
    };
    todos.push(new_todo);
    *global_id += 1;
    HttpResponse::Created().json(json!({
        "message": "TODO added successfully!",
        "todoCount": todos.len()
    }))
}

async fn todos(state: web::Data<AppState>) -> impl Responder {
    let todos = state.todos.lock().unwrap();
    if todos.is_empty() {
        return HttpResponse::NotFound().json(json!({ "message": "No TODOs found yet." }));
    }
    HttpResponse::Ok().json(todos.clone())
}

async fn todo_by_id(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let id: i32 = match path.into_inner().parse() {
        Ok(v) => v,
        Err(_) => return HttpResponse::NotFound().json(json!({ "error": "TODO not found" })),
    };
    let todos = state.todos.lock().unwrap();
    if let Some(t) = todos.iter().find(|t| t.id == id) {
        HttpResponse::Ok().json(t.clone())
    } else {
        HttpResponse::NotFound().json(json!({ "error": "TODO not found" }))
    }
}

async fn mark_as_complete(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let id: i32 = match path.into_inner().parse() {
        Ok(v) => v,
        Err(_) => return HttpResponse::NotFound().json(json!({ "error": "TODO not found" })),
    };
    let mut todos = state.todos.lock().unwrap();
    if let Some(t) = todos.iter_mut().find(|t| t.id == id) {
        t.completed = true;
        let cloned = t.clone();
        HttpResponse::Ok().json(json!({
            "message": "TODO marked as completed!",
            "todo": cloned
        }))
    } else {
        HttpResponse::NotFound().json(json!({ "error": "TODO not found" }))
    }
}

async fn delete_todo_by_id(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let id: i32 = match path.into_inner().parse() {
        Ok(v) => v,
        Err(_) => return HttpResponse::NotFound().json(json!({ "error": "TODO not found" })),
    };
    let mut todos = state.todos.lock().unwrap();
    let original_len = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() < original_len {
        HttpResponse::Ok().json(json!({ "message": "TODO deleted successfully!" }))
    } else {
        HttpResponse::NotFound().json(json!({ "error": "TODO not found" }))
    }
}

#[derive(Debug, Deserialize)]
struct FilterInput {
    status: String,
}

async fn filter_todo(
    query: web::Query<FilterInput>,
    state: web::Data<AppState>,
) -> impl Responder {
    let todos = state.todos.lock().unwrap();
    let filtered: Vec<Todo> = match query.status.as_str() {
        "completed" => todos.iter().filter(|t| t.completed).cloned().collect(),
        "pending" => todos.iter().filter(|t| !t.completed).cloned().collect(),
        _ => {
            return HttpResponse::NotFound()
                .json(json!({ "message": "No TODOs found for the given filter." }));
        }
    };
    HttpResponse::Ok().json(filtered)
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonToSave {
    global_id: i32,
    todos: Vec<Todo>,
}

async fn save_todos(state: web::Data<AppState>) -> impl Responder {
    let file = File::create("todos.json").unwrap();
    let data = JsonToSave {
        global_id: *state.global_id.lock().unwrap(),
        todos: state.todos.lock().unwrap().clone(),
    };
    let json_string = serde_json::to_string_pretty(&data).unwrap();
    let mut bw = BufWriter::new(file);
    bw.write_all(json_string.as_bytes()).unwrap();
    HttpResponse::Ok().json(json!({ "message": "All TODOs saved to file successfully!" }))
}

async fn load_todos(state: web::Data<AppState>) -> impl Responder {
    let data = match fs::read_to_string("todos.json") {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::NotFound().json(json!({ "error": "No saved TODOs found!" }));
        }
    };
    let parsed: JsonToSave = serde_json::from_str(&data).unwrap();
    *state.global_id.lock().unwrap() = parsed.global_id;
    let mut todos = state.todos.lock().unwrap();
    *todos = parsed.todos;
    HttpResponse::Ok().json(json!({
        "message": "TODOs loaded from file successfully!",
        "todoCount": todos.len()
    }))
}

async fn clear_todos(state: web::Data<AppState>) -> impl Responder {
    *state.global_id.lock().unwrap() = 1;
    state.todos.lock().unwrap().clear();
    HttpResponse::Ok().json(json!({ "message": "All TODOs cleared!" }))
}

async fn stats(state: web::Data<AppState>) -> impl Responder {
    let todos = state.todos.lock().unwrap();
    let completed = todos.iter().filter(|t| t.completed).count();
    let pending = todos.len() - completed;
    HttpResponse::Ok().json(json!({
        "total": todos.len(),
        "completed": completed,
        "pending": pending
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        global_id: Mutex::new(1),
        todos: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(hello))
            .route("/add-todo", web::post().to(add_todo))
            .route("/todos", web::get().to(todos))
            .route("/todos/filter", web::get().to(filter_todo))
            .route("/todo/{id}", web::get().to(todo_by_id))
            .route("/todos/{id}/complete", web::put().to(mark_as_complete))
            .route("/todos/{id}", web::delete().to(delete_todo_by_id))
            .route("/save-todos", web::post().to(save_todos))
            .route("/load-todos", web::post().to(load_todos))
            .route("/clear-todos", web::delete().to(clear_todos))
            .route("/stats", web::get().to(stats))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

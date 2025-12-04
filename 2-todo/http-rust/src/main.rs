use actix_web::{
    App, Error, HttpResponse, HttpServer, Responder, Result, dev::ResourcePath,
    error::ErrorNotFound, web,
};
use serde::{Deserialize, Serialize, de};
use serde_json::json;
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    sync::Mutex,
};

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

#[derive(Serialize)]
struct Message {
    message: String,
}
async fn todos(state: web::Data<AppState>) -> Result<impl Responder> {
    let todos = state.todos.lock().unwrap();

    if todos.len() == 0 {
        return Err(actix_web::error::ErrorNotFound(404));
        /*
         TODO:
        Ok(web::Json(Error_not_found {
            message: String::from("No TODOs found yet."),
        }))
         * */
    }

    let mut todos_to_send: Vec<Todo> = Vec::new();

    for val in todos.iter() {
        todos_to_send.push(Todo {
            id: val.id,
            task: val.task.clone(),
            completed: val.completed,
        });
    }
    Ok(web::Json(todos_to_send))
}

#[derive(Serialize)]
struct Todo_by_id_response {
    error: String,
}

async fn todo_by_id(path: web::Path<String>, state: web::Data<AppState>) -> Result<impl Responder> {
    let path = path.into_inner();
    let id: i32 = path.parse().unwrap();
    let todos = state.todos.lock().unwrap();
    let mut found_todo: Option<Todo> = None;

    for todo in todos.iter() {
        if todo.id == id {
            found_todo = Some(todo.clone());
            break;
        }
    }
    let response = Todo_by_id_response {
        error: String::from("TODO not found"),
    };

    if let Some(todo) = found_todo {
        Ok(web::Json(todo))
    } else {
        return Err(actix_web::error::ErrorNotFound(404));
    }

    /*
      TODO: remove this and write the code to return response or with error code and message
      Ok(web::Json(Todo_by_id_response {
      error: String::from("ASdf"),
      }));
    * */
}

#[derive(Serialize)]
struct Mark_as_completed_response {
    message: String,
    todo: Todo,
}

async fn mark_as_complete(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let mut todos = state.todos.lock().unwrap();
    let path = path.into_inner();
    let id: i32 = path.parse().unwrap();

    let mut found_todo: Option<Todo> = None;

    for todo in todos.iter_mut() {
        if todo.id == id {
            todo.completed = true;
            found_todo = Some(todo.clone());
            break;
        }
    }

    if let Some(val) = found_todo {
        Ok(web::Json(Mark_as_completed_response {
            message: String::from("TODO marked as completed!"),
            todo: val,
        }))
    } else {
        return Err(actix_web::error::ErrorNotFound(404));
    }
}

async fn delete_todo_by_id(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let mut todos = state.todos.lock().unwrap();
    let path = path.into_inner();
    let id: i32 = path.parse().unwrap();

    let mut found_todo: Option<Todo> = None;
    let mut todos_to_replace: Vec<Todo> = Vec::new();

    for todo in todos.iter() {
        if todo.id == id {
            found_todo = Some(todo.clone());
        } else {
            todos_to_replace.push(todo.clone());
        }
    }
    *todos = todos_to_replace;

    if let Some(val) = found_todo {
        Ok(web::Json(Message {
            message: String::from("TODO deleted successfully!"),
        }))
    } else {
        return Err(actix_web::error::ErrorNotFound(404));
    }
}

#[derive(Debug, Deserialize)]
struct Filter_todo_input {
    status: String,
}

async fn filter_todo(
    query: web::Query<Filter_todo_input>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let todos = state.todos.lock().unwrap();
    let mut todos_to_return: Vec<Todo> = Vec::new();

    if query.status == "completed" {
        for val in todos.iter() {
            if val.completed {
                todos_to_return.push(val.clone());
            }
        }
    } else if query.status == "pending" {
        for val in todos.iter() {
            if !val.completed {
                todos_to_return.push(val.clone());
            }
        }
    } else {
        return Err(actix_web::error::ErrorNotFound(404));
    }

    Ok(web::Json(todos_to_return))
}

#[derive(Serialize, Deserialize, Debug)]
struct Json_to_save {
    globalId: i32,
    todos: Vec<Todo>,
}
async fn save_todos(state: web::Data<AppState>) -> Result<impl Responder> {
    let file = File::create("todos.json").unwrap();
    // let json_data = json!(*state.todos.lock().unwrap());
    let json_data = json!(Json_to_save {
        globalId: *state.globalId.lock().unwrap(),
        todos: state.todos.lock().unwrap().clone()
    });
    let json_string = serde_json::to_string_pretty(&json_data).unwrap();

    let mut buff_writer = BufWriter::new(file);
    buff_writer.write_all(json_string.as_bytes()).unwrap();

    Ok(web::Json(Message {
        message: String::from("All TODOs saved to file successfully!"),
    }))
}

async fn load_todos(state: web::Data<AppState>) -> Result<impl Responder> {
    let data = fs::read_to_string("todos.json").unwrap();
    let found_data: Json_to_save = serde_json::from_str(&data).unwrap();

    let mut globalId = *state.globalId.lock().unwrap();
    let mut todos = state.todos.lock().unwrap();

    globalId = found_data.globalId;
    *todos = found_data.todos;

    Ok(web::Json(Add_todo_resposne {
        todoCount: todos.len(),
        message: String::from("TODOs loaded from file successfully!"),
    }))
}

async fn clear_todos(state: web::Data<AppState>) -> Result<impl Responder> {
    let mut todos = state.todos.lock().unwrap();
    let mut globalId = state.globalId.lock().unwrap();

    *globalId = 1;
    *todos = Vec::new();

    Ok(web::Json(Message {
        message: String::from("All TODOs cleared!"),
    }))
}

#[derive(Serialize)]
struct Stats_response {
    total: i32,
    completed: i32,
    pending: i32,
}

async fn stats(state: web::Data<AppState>) -> Result<impl Responder> {
    let todos = state.todos.lock().unwrap();
    let mut pending = 0;
    let mut completed = 0;

    for val in todos.iter() {
        if val.completed {
            completed += 1;
        } else {
            pending += 1;
        }
    }
    Ok(web::Json(Stats_response {
        total: pending + completed,
        pending,
        completed,
    }))
}

#[derive(Serialize, Debug, Deserialize, Clone)]
struct Todo {
    id: i32,
    task: String,
    completed: bool,
}

#[derive(Serialize, Debug, Deserialize)]
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

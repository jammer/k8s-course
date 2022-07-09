use std::env;
use std::sync::Mutex;
use std::ops::Deref;
use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[get("/version")]
async fn version(_mutex: web::Data<Mutex<Vec<String>>>) -> impl Responder {
    HttpResponse::Ok().body("Backend")
}

#[get("/todos")]
async fn get_todos(mutex: web::Data<Mutex<Vec<String>>>) -> impl Responder {
    let todos = mutex.lock().expect("Fatal: Mutex error");
    println!("Sending todos");
    HttpResponse::Ok().json(&todos.deref())
}

#[derive(Deserialize)]
struct FormData {
    todo: String,
}

#[post("/todos")]
async fn post_todos(form: web::Form<FormData>, todos: web::Data<Mutex<Vec<String>>>) -> impl Responder {
    todos.lock().expect("Fatal: Mutex error").push(form.todo.to_owned());
    println!("Adding todo");
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port : u16 = env::var("PORT").unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>().expect("Invalid PORT environment variable.");
    println!("Server started in port {}", port);
    let todos_empty : Vec<String> = Vec::new();
    let todos = web::Data::new(Mutex::new(todos_empty));
    HttpServer::new(move || {
        App::new()
            .app_data(todos.clone())
            .service(get_todos)
            .service(post_todos)
            .service(version)
    })
    .bind(("0.0.0.0", port))?
        .run()
        .await
}


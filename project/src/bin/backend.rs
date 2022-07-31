use std::env;
use std::sync::Mutex;
use std::ops::Deref;
use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use tokio_postgres::{NoTls, Error};

#[get("/")]
async fn version() -> impl Responder {
  HttpResponse::Ok().body("Backend")
}

async fn get_todos_db(server: &String) -> Result<Vec<String>,Error> {
  let (client, connection) = tokio_postgres::connect(&server, NoTls).await?;

  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("connection error: {}", e);
    }
  });

  let query = "CREATE TABLE IF NOT EXISTS todos (id SERIAL PRIMARY KEY, task VARCHAR(140))";
  client.batch_execute(query).await?;

  let rows = client.query("SELECT task FROM todos",&[]).await?;
  let mut todos : Vec<String> = Vec::new();
  for r in rows {
    let todo : String = r.get(0);
    todos.push(todo);
  }
  Ok(todos)
}

async fn post_todos_db(server: &String, todo: &String) -> Result<(),Error> {
  let (client, connection) = tokio_postgres::connect(&server, NoTls).await?;
  println!("Posting todo: {}", todo);
  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("connection error: {}", e);
    }
  });

  let query = "CREATE TABLE IF NOT EXISTS todos (id SERIAL PRIMARY KEY, task VARCHAR(140))";
  client.batch_execute(query).await?;

  client.execute("INSERT INTO todos (task) VALUES ($1)",&[todo]).await?;
  println!("Todo added: {}", todo);
  Ok(())
}

#[get("/todos")]
async fn get_todos() -> impl Responder {
  if let Ok(server) = env::var("SERVER") {
    println!("Fetching todos!");
    match get_todos_db(&server).await {
        Ok(todos) => {
            return HttpResponse::Ok().json(&todos.deref());
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("DATABASE ERROR\n{e:#?}"));
        }
    }
  }
  else { return HttpResponse::InternalServerError().body("SERVER variable not set"); }
}

#[derive(Deserialize)]
struct FormData {
todo: String,
}

#[post("/todos")]
async fn post_todos(form: web::Form<FormData>) -> impl Responder {
  if let Ok(server) = env::var("SERVER") {
      if form.todo.len() > 140 {
          println!("Todo length was longer than 140!");
          return HttpResponse::InternalServerError().body("Todo max length is 140!");
      }
      match post_todos_db(&server, &form.todo).await {
        Ok(_) => return HttpResponse::Ok().body(format!("Added todo: {}",form.todo)),
        Err(e) => {
          println!("Todo adding failed! {:?}",e);
          return HttpResponse::InternalServerError().body(format!("Todo not added!\n{:#?}",e));
        }
      }
  }
  else { return HttpResponse::InternalServerError().body("SERVER variable not set"); }
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


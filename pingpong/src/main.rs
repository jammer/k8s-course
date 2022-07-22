use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;
use tokio_postgres::{NoTls, Error};

async fn db(increase : bool) -> Result<i64, Error> {
  let server = env::var("SERVER").unwrap();
  let (client, connection) = tokio_postgres::connect(&server, NoTls).await?;

  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("connection error: {}", e);
    }
  });
  
  let query = "CREATE TABLE IF NOT EXISTS pingpong (id SERIAL PRIMARY KEY)";
  client.batch_execute(query).await?;

  if increase {
    let query = "INSERT INTO pingpong(id) VALUES(DEFAULT)";
    client.batch_execute(query).await?;
  }
  let rows = client.query("SELECT COUNT(id) FROM pingpong",&[]).await?;
  let value : i64 = rows[0].get(0);
  Ok(value)
}

#[get("/pingpong")]
async fn pingpong() -> impl Responder {
  let resp = format!("Pong {}",db(true).await.unwrap());
  HttpResponse::Ok().body(resp)
}

#[get("/pong")]
async fn pong() -> impl Responder {
  let resp = format!("{}",db(false).await.unwrap());
  HttpResponse::Ok().body(resp)
}

#[get("/")]
async fn base() -> impl Responder {
  HttpResponse::Ok().body("Pingpong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let port : u16 = env::var("PORT").unwrap_or_else(|_| "3000".to_string())
    .parse::<u16>().expect("Invalid PORT environment variable.");
  println!("Server started in port {}", port);
  HttpServer::new(|| {
    App::new()
      .service(pingpong)
      .service(pong)
      .service(base)
    })
  .bind(("0.0.0.0", port))?
    .run()
    .await
}


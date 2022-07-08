use uuid::Uuid;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::env;
use std::fs;

#[get("/")]
async fn hello(start: web::Data<Uuid>) -> impl Responder {
    let time = fs::read_to_string("/data/time.txt").unwrap_or_else(|_| "".to_string());
    let resp = format!("{} {}",time.trim(),start.get_ref());
    let counter = reqwest::get("http://pingpong-svc/pong").await.unwrap().text().await.unwrap();
    println!("{}",resp);
    HttpResponse::Ok().body(resp + "\nPing / Pongs: " + &counter)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port : u16 = env::var("PORT").unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>().expect("Invalid PORT environment variable.");
    println!("Server started in port {}", port);
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Uuid::new_v4()))
            .service(hello)
    })
    .bind(("0.0.0.0", port))?
        .run()
        .await
}


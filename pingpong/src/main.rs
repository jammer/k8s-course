use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;
use std::fs;

#[get("/pingpong")]
async fn hello() -> impl Responder {
    let mut counter:u64 = fs::read_to_string("/data/pong.txt").unwrap_or_else(|_| "0".to_string()).trim().parse().unwrap();
    counter += 1;
    fs::write("/data/pong.txt",counter.to_string()).unwrap();
    let resp = format!("Pong {}",counter);
    HttpResponse::Ok().body(resp)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port : u16 = env::var("PORT").unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>().expect("Invalid PORT environment variable.");
    println!("Server started in port {}", port);
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("0.0.0.0", port))?
        .run()
        .await
}


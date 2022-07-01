use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;
use rand::{distributions::Alphanumeric, Rng};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[get("/pingpong")]
async fn hello() -> impl Responder {
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
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


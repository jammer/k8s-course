use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::env;
use rand::{distributions::Alphanumeric, Rng};

fn random() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

#[get("/")]
async fn hello(start: web::Data<String>) -> impl Responder {
    println!("Request!");
    let resp = format!("Hello {} from {}",random(),start.get_ref());
    HttpResponse::Ok().body(resp)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port : u16 = env::var("PORT").unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>().expect("Invalid PORT environment variable.");
    println!("Server started in port {}", port);
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(random()))
            .service(hello)
    })
    .bind(("0.0.0.0", port))?
        .run()
        .await
}


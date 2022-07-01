use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::env;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;
use actix_web::http::header::ContentType;

fn random() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

#[get("/")]
async fn hello(start: web::Data<String>) -> impl Responder {
    let dt = chrono::Utc::today();
    let fmt = format!("/data/{}.jpg",dt);
    let file = std::path::Path::new(&fmt);
    if !file.exists() {
        let bytes = reqwest::get("https://picsum.photos/300").await.unwrap().bytes().await.unwrap();
        fs::write(file,bytes);
    }
    println!("Request!");
    let html = r#"
<img src="picture.jpg"><br>
<input type="text" maxlength="140">
<input type="button" value="Create TODO">
<ul>
	<li>Todo 1</li>
	<li>Todo 2</li>
</ul>
"#;
    HttpResponse::Ok().insert_header(ContentType::html()).body(html)
}

#[get("/picture.jpg")]
async fn picture(start: web::Data<String>) -> impl Responder {
    let dt = chrono::Utc::today();
    let fmt = format!("/data/{}.jpg",dt);
    let file = std::path::Path::new(&fmt);
    if !file.exists() {
        println!("Picture not here");
        return HttpResponse::NotFound().body("");
    }
    HttpResponse::Ok().body(fs::read(file).unwrap())
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
            .service(picture)
    })
    .bind(("0.0.0.0", port))?
        .run()
        .await
}


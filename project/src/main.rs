use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;
use std::fs;
use actix_web::http::header::ContentType;

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body("Frontend")
}

#[get("/")]
async fn hello() -> impl Responder {
    let dt = chrono::Utc::today();
    let fmt = format!("/data/{}.jpg",dt);
    let file = std::path::Path::new(&fmt);
    if !file.exists() {
        println!("Fetching new image");
        let req = reqwest::get("https://picsum.photos/300").await;
        match req {
            Ok(_) => {}
            Err(_) => { println!("Error fetching image"); panic!("Error fetching image") }
        }
        let bytes = req.unwrap().bytes().await.unwrap();
        match fs::write(file,bytes) {
            Ok(_) => println!("Wrote new image {:?}",file),
            Err(_) => println!("Error writing file, permissions?"),
        }
    } else {
        println!("Already have picture {:?}",file);
    }
    println!("Request!");
    let req = reqwest::get("http://backend-svc/todos").await;
    match req {
        Ok(_) => {}
        Err(_) => { println!("Error fetching todos"); panic!("Error fetching todos") }
    }
    let json = req.unwrap().json::<Vec<String>>().await.unwrap();
    let mut html = r#"
<img src="picture.jpg"><br>
<form method="post" action="/todos">
<input type="text" name="todo" maxlength="140">
<input type="submit" value="Create TODO">
</form>
<ul>"#.to_owned();
    for i in json {
       html = html + "<li>" + &i + "</li>"; 
    }
    html = html + "</ul>";
    HttpResponse::Ok().insert_header(ContentType::html()).body(html)
}

#[get("/picture.jpg")]
async fn picture() -> impl Responder {
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
    let port : u16 = env::var("PORT").unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>().expect("Invalid PORT environment variable.");
    println!("Server started in port {}", port);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(picture)
            .service(version)
    })
    .bind(("0.0.0.0", port))?
        .run()
        .await
}


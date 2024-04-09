use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[get("/StartField")]
// async fn start_field() -> Result<fs::NamedFile> {

//     // HttpResponse::Ok().body()
//     // send html with p5js code at /pagefi.

//     Ok(fs::NamedFile::open("../../pagefile/index.html")?)
// }

// #[get("/StartField")]
// async fn start_field() -> Result<fs::NamedFile, error::Error> {
//     // Serve index.html file when /StartField is accessed
//     Ok(fs::NamedFile::open("./static/index.html")?)
// }
#[get("/StarField")]
async fn start_field() -> actix_web::Result<fs::NamedFile> {
    // Serve index.html file when /StartField is accessed
    // this path is relative path from cargo run
    Ok(fs::NamedFile::open("../pagefile/index.html")?)
}



#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(start_field) // Make sure to add the service
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            // Serving static files from "./static" directory
            .service(fs::Files::new("/pagefile", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
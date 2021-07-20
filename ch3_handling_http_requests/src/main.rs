use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("function is firing");
        let app = App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet));
        app
    })
    .bind("127.0.0.1:8000")?
    .workers(3)
    .run()
    .await
}

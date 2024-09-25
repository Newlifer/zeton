use actix_web::{dev::AppService, web, App, Error, HttpResponse, HttpServer};
use dotenvy::dotenv;

pub async fn test() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("I'm teapot"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server = HttpServer::new(move || {
        App::new().service(
            web::resource("/users")
                .route(web::get().to(test)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run();
    println!("Server running!");

    server.await
}

use actix_web::{get, post, delete, web, App, Error, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::NoTls;
use deadpool_postgres::{Config, Runtime, Client, Pool};
use dotenvy::dotenv;


#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "tokens")]
struct Token {
    pub external_key: String,
    pub generated_key: String,
    pub created: String,
    pub applied: String,
    pub ttl: u32,
}


#[derive(Deserialize)]
struct NewToken {
    token: String,
    lifetime: u32,
}

#[get("/read/{token}")]
async fn read_token(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let token = path.into_inner();
    Ok(HttpResponse::Ok().body(token))
}

#[post("/create")]
async fn create_token(info: web::Json<NewToken>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(info.token.clone()))
}

#[delete("/delete")]
async fn delete_token(info: web::Json<NewToken>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(info.token.clone()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server = HttpServer::new(move
        || { App::new().service(read_token).service(create_token).service(delete_token) }
    )
    .bind("127.0.0.1:8080")?
    .run();

    println!("Server running!");

    server.await
}

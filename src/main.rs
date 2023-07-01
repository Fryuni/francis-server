use std::borrow::Cow;

use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};
use log::debug;

#[derive(Debug, Serialize, Deserialize)]
struct AppliedItem<'a> {
    id: Cow<'a, str>,
    name: Cow<'a, str>,
    amount: i32,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/set_items")]
async fn set_items(Json(body): Json<Vec<AppliedItem<'_>>>) -> impl Responder {
    println!("{body:?}");

    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "francis=debug,actix_web=info");
    pretty_env_logger::init();

    debug!("Starting server");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(set_items)
    })
    .bind(("0.0.0.0", 8088))?
    .run()
    .await
}

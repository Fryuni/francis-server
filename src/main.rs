use std::collections::HashMap;

use actix_web::middleware::Logger;
use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer, Responder};
use color_eyre::eyre::{eyre, WrapErr};
use firestore::FirestoreDb;
use gcloud_sdk::GoogleEnvironment;
use log::{debug, error};

use crate::db::DbHandle;
use crate::model::AppliedItem;

mod db;
mod model;

#[get("/")]
async fn hello(db: DbHandle) -> impl Responder {
    let users = match db.list_docs().await {
        Ok(users) => users,
        Err(err) => {
            error!("Error listing docs: {err}");
            return HttpResponse::InternalServerError().body("Failed to list docs");
        }
    };

    let users = users.iter()
        .map(firestore::firestore_document_to_serializable::<HashMap<String, serde_json::Value>>)
        .collect::<Result<Vec<_>, _>>().unwrap();

    HttpResponse::Ok().json(users)
}

#[post("/set_items")]
async fn set_items(Json(body): Json<Vec<AppliedItem<'_>>>) -> impl Responder {
    println!("{body:?}");

    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "francis=debug,actix_web=debug,firestore=debug,gcloud_sdk=debug,warn",
    );
    pretty_env_logger::init();

    let project_id = GoogleEnvironment::detect_google_project_id()
        .await
        .ok_or(eyre!("Google project ID must be set in the environment"))?;
    let db = FirestoreDb::new(project_id)
        .await
        .wrap_err("Failed to start Firestore connection")?;

    debug!("Starting server");
    HttpServer::new(move || {
        debug!("Instantiating application");
        App::new()
            .app_data(db.clone())
            .wrap(Logger::default())
            .service(hello)
            .service(set_items)
    })
    .bind((
        "0.0.0.0",
        std::env::var("PORT")
            .wrap_err("reading PORT")
            .and_then(|port| port.parse().wrap_err("parsing PORT"))
            .unwrap_or(8080),
    ))?
    .run()
    .await?;

    Ok(())
}

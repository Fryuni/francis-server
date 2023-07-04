use std::collections::BTreeMap;

use actix_web::middleware::Logger;
use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer, Responder};
use color_eyre::eyre::{eyre, WrapErr};
use firestore::FirestoreDb;
use gcloud_sdk::GoogleEnvironment;
use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::db::DbHandle;
use crate::model::AppliedItem;

mod db;
mod model;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(alias = "_firestore_id")]
    id: String,
    role: String,
}

#[get("/")]
async fn hello(db: DbHandle) -> impl Responder {
    let items = match db.list_items().await {
        Ok(items) => items,
        Err(err) => {
            error!("Error listing docs: {err}");
            return HttpResponse::InternalServerError().body("Failed to list docs");
        }
    };

    HttpResponse::Ok().json(items)
}

#[post("/set_items")]
async fn set_items<'a>(db: DbHandle, Json(body): Json<Vec<AppliedItem<'a>>>) -> impl Responder {
    let mut entries: BTreeMap<String, AppliedItem<'a>> = BTreeMap::new();

    for item in body {
        match entries.entry(item.id.clone().into_owned()) {
            std::collections::btree_map::Entry::Vacant(entry) => { entry.insert(item); },
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                entry.get_mut().amount += item.amount;
            }
        }
    }

    let body = entries.into_values().collect();

    match db.set_items(body).await {
        Ok(_) => {}
        Err(err) => {
            error!("Error saving items data: {err:?}");
        }
    };

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

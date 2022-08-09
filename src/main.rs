mod adapters;
mod domain;
mod ports;
mod use_cases;

use actix_web::{web, App, HttpServer};
use core::panic;
use env_logger;
use std::env::{set_var, var_os};

use crate::{
    adapters::repositories::mem_db::taxon_fetching::TaxonFetchingMemBdRepository,
    ports::api::main::resolve_taxid,
    use_cases::load_source_dump_database::load_source_dump_database,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configure logger
    set_var("RUST_LOG", "debug");
    set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    let db_file_path = match var_os("DATABASE_PATH") {
        Some(path) => path.into_string().unwrap(),
        None => String::from("/home/samuel-elias/study-projects/rust/biotax/src/assets/names-tab-200.dmp"),
    };

    let db_result = load_source_dump_database(db_file_path.as_str());

    if db_result.is_err() {
        panic!(
            "Unexpected error occurred on load database: {}",
            db_result.unwrap_err()
        )
    }

    let db = db_result.unwrap();

    //let repo = TaxonFetchingMemBdRepository::new(db.ok().unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(TaxonFetchingMemBdRepository::new(
                db.clone(),
            )))
            .service(resolve_taxid)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

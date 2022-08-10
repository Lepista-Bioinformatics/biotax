mod adapters;
mod domain;
mod ports;
mod use_cases;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use core::panic;
use env_logger;
use std::{
    env::{set_var, var_os},
    sync::Arc,
};

use crate::adapters::repositories::mem_db::taxon_fetching::TaxonFetchingMemDbRepositoryParameters;
use crate::{
    adapters::repositories::mem_db::taxon_fetching::TaxonFetchingMemDbRepository,
    ports::api::{
        endpoints::{health, resolve_taxid},
        modules::TaxonFetchingModule,
    },
    use_cases::load_source_dump_database::load_source_dump_database,
};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    // Configure logger from environment variables.
    set_var("RUST_LOG", "debug");
    set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    // Collect database path from environment variable.
    let db_file_path = match var_os("DATABASE_PATH") {
        Some(path) => path.into_string().unwrap(),
        None => String::from("/home/samuel-elias/study-projects/rust/biotax/src/assets/names-tab-200.dmp"),
    };

    // Load database from `DATABASE_PATH` environment variable.
    let db_result = load_source_dump_database(db_file_path.as_str());

    if db_result.is_err() {
        panic!(
            "Unexpected error occurred on load database: {}",
            db_result.unwrap_err()
        )
    }

    // Start a module for dependency injection.
    let module = Arc::new(
        TaxonFetchingModule::builder()
            .with_component_parameters::<TaxonFetchingMemDbRepository>(
                TaxonFetchingMemDbRepositoryParameters {
                    db: db_result.unwrap().clone(),
                },
            )
            .build(),
    );

    // Configure the server configuration.
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(module.clone())
            .service(resolve_taxid)
            .service(health)
    });

    // Fire the server.
    server.bind(("0.0.0.0", 8080))?.run().await
}

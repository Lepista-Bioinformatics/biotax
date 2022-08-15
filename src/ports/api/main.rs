extern crate biotax;
mod endpoints;
mod modules;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use biotax::settings::ARANGODB_COLLECTION_NAME;
use biotax::{
    adapters::repositories::arango_db::taxon_fetching::{
        TaxonFetchingArangodbRepository,
        TaxonFetchingArangodbRepositoryParameters,
    },
    settings::ARANGODB_DATABASE_NAME,
};
use env_logger;
use log::info;
use rust_arango::Connection;
use std::{env::var_os, sync::Arc};

use crate::endpoints::{health, resolve_taxid};
use crate::modules::TaxonFetchingModule;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    // Build logger
    env_logger::init();

    // Build repository
    info!("Get API and database credentials");
    let (user, pass, db_address, ip, port) = get_db_conn_credentials();

    info!("Build database connection.");
    let conn = Connection::establish_basic_auth(&db_address, &user, &pass)
        .await
        .unwrap();

    let db = conn.db(ARANGODB_DATABASE_NAME).await.unwrap();

    info!("Start a module for dependency injection.");
    let module = Arc::new(
        TaxonFetchingModule::builder()
            .with_component_parameters::<TaxonFetchingArangodbRepository>(
                TaxonFetchingArangodbRepositoryParameters {
                    db,
                    collection: &ARANGODB_COLLECTION_NAME,
                },
            )
            .build(),
    );

    info!("Set the server configuration.");
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(module.clone())
            .service(resolve_taxid)
            .service(health)
    });

    info!("Fire the server.");
    server.bind((ip, port))?.run().await
}

fn get_db_conn_credentials() -> (String, String, String, String, u16) {
    let arangodb_username = match var_os("ARANGODB_USERNAME") {
        Some(path) => path.into_string().unwrap(),
        None => String::from("root"),
    };

    let arangodb_password = match var_os("ARANGODB_PASSWORD") {
        Some(path) => path.into_string().unwrap(),
        None => String::from("arangodb"),
    };

    let arangodb_address = match var_os("ARANGODB_ADDRESS") {
        Some(path) => path.into_string().unwrap(),
        None => String::from("http://localhost:8529"),
    };

    let service_ip = match var_os("SERVICE_IP") {
        Some(path) => path.into_string().unwrap(),
        None => String::from("0.0.0.0"),
    };

    let service_port = match var_os("SERVICE_PORT") {
        Some(path) => path.into_string().unwrap().parse::<u16>().unwrap(),
        None => 8080,
    };

    (
        arangodb_username,
        arangodb_password,
        arangodb_address,
        service_ip,
        service_port,
    )
}

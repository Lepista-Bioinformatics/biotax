fn main() {}

//extern crate biotax;
//mod endpoints;
//mod modules;

//use actix_web::middleware::Logger;
//use actix_web::{App, HttpServer};
//use biotax::adapters::repositories::kv_db::taxon_fetching::{
//    CustomPickleDb, TaxonFetchingKvDbRepository,
//    TaxonFetchingKvDbRepositoryParameters,
//};
//use env_logger;
//use log::info;
//use pickledb::{PickleDb, SerializationMethod};
//use rust_arango::Connection;
//use std::{
//    env::{set_var, var_os},
//    sync::Arc,
//};

//use crate::endpoints::{health, resolve_taxid};
//use crate::modules::TaxonFetchingModule;

//#[actix_web::main]
//pub async fn main() -> std::io::Result<()> {
//    // Configure logger from environment variables.
//    set_var("RUST_LOG", "debug");
//    set_var("RUST_BACKTRACE", "full");
//    env_logger::init();
//
//    info!("Collect database path from environment variable.");
//    let db_file_path = match var_os("DATABASE_PATH") {
//        Some(path) => path.into_string().unwrap(),
//        None => String::from("assets/names-tab-200.dmp"),
//    };
//
//    let arangodb_username = match var_os("ARANGODB_USERNAME") {
//        Some(path) => path.into_string().unwrap(),
//        None => String::from("assets/names-tab-200.dmp"),
//    };
//
//    let arangodb_password = match var_os("ARANGODB_PASSWORD") {
//        Some(path) => path.into_string().unwrap(),
//        None => String::from("assets/names-tab-200.dmp"),
//    };
//
//    info!("Collect service port from environment variable.");
//    let service_port = match var_os("SERVICE_PORT") {
//        Some(path) => path.into_string().unwrap().parse::<u16>().unwrap(),
//        None => 8080,
//    };
//
//    info!("Load database from `DATABASE_PATH` environment variable.");
//    info!("{:?}", db_file_path);
//    let conn = Connection::establish_jwt(
//        "http://localhost:8529",
//        &arangodb_password,
//        &arangodb_password,
//    )
//    .await
//    .unwrap();
//    let readonly_db =
//        PickleDb::load_read_only(db_file_path, SerializationMethod::Json)
//            .unwrap();
//
//    info!("Start a module for dependency injection");
//    let module = Arc::new(
//        TaxonFetchingModule::builder()
//            .with_component_parameters::<TaxonFetchingKvDbRepository>(
//                TaxonFetchingKvDbRepositoryParameters {
//                    db: CustomPickleDb { conn: readonly_db },
//                },
//            )
//            .build(),
//    );
//
//    info!("Set the server configuration.");
//    let server = HttpServer::new(move || {
//        App::new()
//            .wrap(Logger::default())
//            .wrap(Logger::new("%a %{User-Agent}i"))
//            .app_data(module.clone())
//            .service(resolve_taxid)
//            .service(health)
//    });
//
//    info!("Fire the server.");
//    server.bind(("0.0.0.0", service_port))?.run().await
//}

//fn get_database_credentials() -> Result<> {}

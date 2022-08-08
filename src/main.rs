mod adapters;
mod domain;
mod ports;
mod use_cases;

use std::env::var_os;

use actix_web::{App, HttpServer};

use crate::{
    adapters::repositories::mem_db::taxon_fetching::TaxonFetchingMemBdRepository,
    ports::api::main::resolve_taxid,
    use_cases::load_source_dump_database::load_source_dump_database,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_file_path = match var_os("DATABASE_PATH") {
        Some(path) => path.into_string().unwrap(),
        None => String::from("/home/samuel-elias/study-projects/rust/biotax/src/assets/names-tab-200.dmp"),
    };

    let db = load_source_dump_database(db_file_path.as_str());
    let repo = TaxonFetchingMemBdRepository::new(db.unwrap());

    HttpServer::new(move || {
        App::new().app_data(repo.clone()).service(resolve_taxid)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

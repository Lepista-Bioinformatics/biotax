extern crate biotax;

use quicli::prelude::CliResult;
use structopt::StructOpt;

use std::env::{set_var, var_os};

use biotax::{
    adapters::repositories::arango_db::taxon_registration::TaxonRegistrationArangoDbRepository,
    settings::{ARANGODB_COLLECTION_NAME, ARANGODB_DATABASE_NAME},
    use_cases::build_composed_database::build_composed_database,
};

#[derive(Debug, StructOpt)]
struct Cli {
    // This is the names.dmp file system path
    names: String,
    // This is the nodes.dmp file system path
    nodes: String,
    // This is the division.dmp file system path
    division: String,
}

#[tokio::main]
pub async fn main() -> CliResult {
    // Build logger
    set_var("RUST_LOG", "debug");
    env_logger::init();

    // Build repository
    let (user, pass, address) = configure_cli();

    let repo = TaxonRegistrationArangoDbRepository {
        //reqwest_client: Client::new(),
        service_url: address,
        username: user,
        password: Some(pass),
    };

    // Build cli
    let args = Cli::from_args();

    // Execute the database builder
    let building_response = build_composed_database(
        &args.names.as_str(),
        &args.nodes.as_str(),
        &args.division.as_str(),
        &repo,
    )
    .await;

    if building_response.is_err() {
        panic!("Unexpected error detected on build database.");
    }

    Ok(())
}

fn configure_cli() -> (String, String, String) {
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

    let address = format!(
        "{}/_db/{}/_api/document/{}",
        arangodb_address, ARANGODB_DATABASE_NAME, ARANGODB_COLLECTION_NAME,
    );

    (arangodb_username, arangodb_password, address)
}

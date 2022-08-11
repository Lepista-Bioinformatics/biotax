extern crate biotax;

use futures::executor::block_on;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use quicli::prelude::CliResult;
use structopt::StructOpt;

use biotax::{
    adapters::repositories::kv_db::taxon_registration::TaxonRegistrationKvDbRepository,
    settings::PICKLE_DB_PATH,
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

pub fn main() -> CliResult {
    let args = Cli::from_args();

    let registration_repo = TaxonRegistrationKvDbRepository {};

    let mut db = PickleDb::new(
        PICKLE_DB_PATH,
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    );

    let building_response = block_on(build_composed_database(
        &args.names.as_str(),
        &args.nodes.as_str(),
        &args.division.as_str(),
        &registration_repo,
        &mut db,
    ));

    if building_response.is_err() {
        panic!("Unexpected error detected on build database.");
    }

    Ok(())
}

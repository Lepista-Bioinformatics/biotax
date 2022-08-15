use crate::{
    domain::{
        entities::taxon_registration::TaxonRegistration,
        utils::errors::MappedErrors,
    },
    settings::TMP_NAMES_JOINED_DATAFRAME,
    use_cases::load_source_dump_database::load_source_dump_database,
};

use polars::prelude::*;
use std::{
    fs::{remove_file, File},
    result::Result,
};

use log::{error, info};

use super::load_source_dump_database::TaxonDatabase;

pub async fn build_composed_database(
    names_file_path: &str,
    nodes_file_path: &str,
    division_file_path: &str,
    taxon_registration_repo: &dyn TaxonRegistration,
) -> Result<bool, MappedErrors> {
    info!("Building reference database");
    let filtered_dataframe = load_reference_dumps(
        names_file_path,
        nodes_file_path,
        division_file_path,
    );

    info!("Persist temporary file locally");
    let output_file = File::create(&TMP_NAMES_JOINED_DATAFRAME).unwrap();
    CsvWriter::new(&output_file)
        .has_header(true)
        .with_delimiter(b'\t')
        .finish(&mut filtered_dataframe.unwrap())
        .unwrap();

    info!("Re-loading as a `TaxonDatabase`");
    let building_response =
        load_source_dump_database(&TMP_NAMES_JOINED_DATAFRAME).await;

    if building_response.is_err() {
        let msg = "Unexpected error detected on build database.";
        error!("{}", &msg);
        return Err(MappedErrors::new(&msg, None, None));
    }

    info!("Removing temporary artifact");
    let remove_response = remove_file(&TMP_NAMES_JOINED_DATAFRAME);

    if remove_response.is_err() {
        let msg = "Unexpected error detected on remove temporary artifacts.";
        error!("{}", &msg);
        return Err(MappedErrors::new(&msg, None, None));
    }

    info!("Persisting response to database");
    let persistence_response =
        persist_database(building_response.unwrap(), taxon_registration_repo)
            .await;

    if persistence_response.is_err() {
        let msg = "Unexpected error detected on persist record to database.";
        error!("{}", &msg);
        return Err(MappedErrors::new(&msg, None, None));
    }

    info!("All is done");
    Ok(true)
}

async fn persist_database(
    taxa: TaxonDatabase,
    taxon_registration_repo: &dyn TaxonRegistration,
) -> Result<bool, MappedErrors> {
    for (taxid, taxon) in taxa {
        info!("Processing taxid {}", taxid);

        let registration_response =
            taxon_registration_repo.get_or_create(taxid, taxon).await;

        if registration_response.is_err() {
            let msg =
                "Unexpected error detected on persist record to database.";
            error!("{}", &msg);
            return Err(MappedErrors::new(&msg, None, None));
        }
    }

    Ok(true)
}

fn load_reference_dumps(
    names_path: &str,
    nodes_path: &str,
    division_path: &str,
) -> Result<DataFrame, MappedErrors> {
    let nodes_df = get_nodes_dataframe(nodes_path);

    if nodes_df.is_err() {
        return Err(MappedErrors::new(
            "Unexpected error detected on load nodes.",
            None,
            None,
        ));
    }

    let division_df = get_division_dataframe(division_path);

    if division_df.is_err() {
        return Err(MappedErrors::new(
            "Unexpected error detected on load division.",
            None,
            None,
        ));
    }

    let nodes_with_division = nodes_df?.join(
        &division_df.unwrap(),
        ["division_id"],
        ["division_id"],
        JoinType::Inner,
        None,
    );

    if nodes_with_division.is_err() {
        return Err(MappedErrors::new(
            "Unexpected error detected on merge division and nodes.",
            None,
            None,
        ));
    }

    let names_df = get_names_dataframe(names_path);

    let full_dataframe = names_df?.join(
        &nodes_with_division.unwrap(),
        ["tax_id"],
        ["tax_id"],
        JoinType::Inner,
        None,
    );

    if full_dataframe.is_err() {
        return Err(MappedErrors::new(
            "Unexpected error detected on build final dataframe.",
            None,
            None,
        ));
    }

    let filtered_dataframe = full_dataframe.unwrap().select([
        "tax_id",
        "name_txt",
        "unique_name",
        "name_class",
        "parent_tax_id",
        "rank",
        "division_cde",
    ]);

    Ok(filtered_dataframe.unwrap())
}

fn get_names_dataframe(path: &str) -> Result<DataFrame, MappedErrors> {
    let column_definitions = vec![
        // 1 - the id of node associated with this name
        ("tax_id".to_string(), DataType::Int64),
        // 2 - name itself
        ("name_txt".to_string(), DataType::Utf8),
        // 3 - the unique variant of this name if name not unique
        ("unique_name".to_string(), DataType::Utf8),
        // 4 - (synonym, common name, ...)
        ("name_class".to_string(), DataType::Utf8),
    ];

    load_named_dataframe(path, column_definitions, vec![])
}

fn get_nodes_dataframe(path: &str) -> Result<DataFrame, MappedErrors> {
    let column_definitions = vec![
        // 1 - node id in GenBank taxonomy database
        ("tax_id".to_string(), DataType::Int64),
        // 2 - parent node id in GenBank taxonomy database
        ("parent_tax_id".to_string(), DataType::Int64),
        // 3 - rank of this node (superkingdom, kingdom, ...)
        ("rank".to_string(), DataType::Utf8),
        // 4 - locus-name prefix; not unique
        ("embl_code".to_string(), DataType::Utf8),
        // 5 - see division.dmp file
        ("division_id".to_string(), DataType::Int64),
        // 6 - 1 if node inherits division from parent
        // ("inherited_div_flag".to_string(), DataType::Int32),
        // 7 - see gencode.dmp file
        // ("genetic_code_id".to_string(), DataType::Int64),
        // 8 - 1 if node inherits genetic code from parent
        // ("inherited_GC_flag".to_string(), DataType::Int32),
        // 9 - see gencode.dmp file
        // ("mitochondrial_genetic_code_id".to_string(), DataType::Int64),
        // 10 - 1 if node inherits mitochondrial gencode from parent
        // ("inherited_MGC_flag".to_string(), DataType::Int32),
        // 11 - 1 if name is suppressed in GenBank entry lineage
        // ("genBank_hidden_flag".to_string(), DataType::Int32),
        // 12 - 1 if this subtree has no sequence data yet
        // ("hidden_subtree_root_flag".to_string(), DataType::Int32),
        // 13 - free-text comments and citations
        // ("comments".to_string(), DataType::Utf8),
        // 14 - see gencode.dmp file
        // ("plastid_genetic_code_id".to_string(), DataType::Int64),
        // 15 - 1 if node inherits plastid gencode from parent
        // ("inherited_PGC_flag".to_string(), DataType::Int32),
        // 16 - 1 if species in the node's lineage has formal name
        // ("specified_species".to_string(), DataType::Int64),
        // 17 - see gencode.dmp file
        // ("hydrogenosome_genetic_code_id".to_string(), DataType::Int64),
        // 18 - 1 if node inherits hydrogenosome gencode from parent
        // ("inherited_HGC_flag".to_string(), DataType::Int32),
    ];

    load_named_dataframe(
        path,
        column_definitions,
        vec!["embl_code".to_string()],
    )
}

fn get_division_dataframe(path: &str) -> Result<DataFrame, MappedErrors> {
    let column_definitions = vec![
        // 1 - taxonomy database division id
        ("division_id".to_string(), DataType::Int64),
        // 2 - GenBank division code (three characters)
        ("division_cde".to_string(), DataType::Utf8),
        // 3 - GenBank division names
        // ("division_name".to_string(), DataType::Utf8),
        // 4 - Free comments
        // ("comments".to_string(), DataType::Int64),
    ];

    load_named_dataframe(path, column_definitions, vec![])
}

fn load_named_dataframe(
    path: &str,
    column_definitions: Vec<(String, DataType)>,
    exclude_list: Vec<String>,
) -> Result<DataFrame, MappedErrors> {
    // initialize dataframe schema
    let mut schema = Schema::new();

    // Map definitions to schema
    for (name, column_type) in &column_definitions {
        schema.with_column(name.to_owned(), column_type.to_owned())
    }

    // Collect column names
    let mut columns_names: Vec<String> = [].to_vec();

    for (name, _) in &column_definitions {
        if exclude_list.contains(name) {
            continue;
        }

        columns_names.push(name.to_owned());
    }

    // Load dataframe
    load_table(path, schema, columns_names)
}

fn load_table(
    path: &str,
    schema: Schema,
    columns: Vec<String>,
) -> Result<DataFrame, MappedErrors> {
    let reading = CsvReader::from_path(path);

    if reading.is_err() {
        return Err(MappedErrors::new(
            "Unexpected error occurred on load table.",
            None,
            None,
        ));
    }

    Ok(reading
        .unwrap()
        .with_delimiter(b'\t')
        .has_header(false)
        .with_schema(&schema)
        .with_columns(Some(columns))
        .finish()
        .unwrap())
}

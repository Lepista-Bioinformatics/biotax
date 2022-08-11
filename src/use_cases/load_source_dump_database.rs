use std::{collections::BTreeMap, fmt::Error};

use crate::domain::dtos::taxon::TaxonDTO;
use csv::ReaderBuilder;
use log::info;

pub type TaxonDatabase = BTreeMap<String, Vec<TaxonDTO>>;

pub fn load_source_dump_database(
    source_data_frame: &str,
) -> Result<TaxonDatabase, Error> {
    // Initialize the CSV reader from a system path.
    let reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(source_data_frame);

    if reader.is_err() {
        return Err(Error);
    }

    let mut taxa: TaxonDatabase = TaxonDatabase::new();

    for line in reader.unwrap().records() {
        // Check errors in line response
        if line.is_err() {
            return Err(Error);
        }

        // Unpack records pieces from line content
        let record = line.unwrap();
        let tax_id = record[0].parse::<i64>().to_owned().unwrap();
        let tax_name = record[1].parse::<String>().unwrap().to_owned();
        let unique_name = record[2].parse::<String>().unwrap().to_owned();
        let name_class = record[3].parse::<String>().unwrap().to_owned();

        if name_class != "scientific name" {
            continue;
        }

        // Build taxon element
        let mut taxon = TaxonDTO {
            tax_id,
            tax_name: tax_name,
            unique_name: None,
            name_class: name_class,
        };

        if unique_name != "" {
            taxon.unique_name = Some(unique_name);
        }

        // Update the taxa map
        let prev_value = taxa.get(&tax_id.to_string());
        let mut new_value: Vec<TaxonDTO> = [].to_vec();

        if prev_value.is_some() {
            new_value = prev_value.unwrap().to_vec();
        }

        new_value.push(taxon);
        taxa.insert(tax_id.to_string(), new_value);
    }

    info!("Loaded {} records to memory.", taxa.len());

    Ok(taxa)
}

// ? --------------------------------------------------------------------------
// ? Tests
// ? --------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_source_dump_as_hashmap_vector_test() {
        let load_response = load_source_dump_database(
            "/home/samuel-elias/study-projects/rust/biotax/src/assets/names-tab-200.dmp"
        );

        assert_eq!(load_response.is_ok(), true);

        let db = load_response.unwrap();

        for (key, value) in &db {
            println!("key: {}\nvalye: {:?}\n", key, value);
        }

        println!("Inserted {} tax_ids", db.keys().len());
        println!("Value found {:?}", db.get(&999.to_string()));
    }
}

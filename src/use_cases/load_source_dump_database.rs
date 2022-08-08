use std::collections::BTreeMap;

use crate::domain::dtos::taxon::TaxonDTO;
use csv::{Error, ReaderBuilder};

pub type TaxonDatabase = BTreeMap<String, Vec<TaxonDTO>>;

#[allow(dead_code)]
pub fn load_source_dump_database(
    source_data_frame: &str,
) -> Result<TaxonDatabase, Error> {
    // Initialize the CSV reader from a system path.
    let reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(source_data_frame);

    if reader.is_err() {
        return Err(reader.unwrap_err());
    }

    let mut taxa: TaxonDatabase = TaxonDatabase::new();

    for line in reader.unwrap().records() {
        // Check errors in line response
        if line.is_err() {
            return Err(line.unwrap_err());
        }

        // Unpack records pieces from line content
        let record = line.unwrap();
        let tax_id = record[0].parse::<i64>().to_owned().unwrap();
        let tax_name = record[1].parse::<String>().to_owned().unwrap();
        let unique_name = record[2].parse::<String>().to_owned().unwrap();
        let name_class = record[3].parse::<String>().to_owned().unwrap();

        // Build taxon element
        let mut taxon = TaxonDTO {
            tax_id,
            tax_name,
            unique_name: None,
            name_class,
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

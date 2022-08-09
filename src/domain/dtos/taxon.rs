use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxonDTO {
    pub tax_id: i64,
    pub tax_name: String,
    pub unique_name: Option<String>,
    pub name_class: String,
}

// ? --------------------------------------------------------------------------
// ? Tests
// ? --------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::TaxonDTO;

    #[test]
    fn create_taxon() {
        let tax_id: i64 = 5502;
        let tax_name = String::from("Curvularia");
        let unique_name = String::from("null");
        let name_class = String::from("scientific name");

        let taxon = TaxonDTO {
            tax_id,
            tax_name,
            unique_name: Some(unique_name),
            name_class,
        };

        assert_eq!(taxon.tax_id, tax_id);
    }
}

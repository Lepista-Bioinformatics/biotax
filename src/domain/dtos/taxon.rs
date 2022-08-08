use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
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
            tax_name: tax_name.clone(),
            unique_name: Some(unique_name.clone()),
            name_class: name_class.clone(),
        };

        assert_eq!(taxon.tax_id, tax_id);
        assert_eq!(taxon.tax_name, tax_name);
        assert_eq!(taxon.name_class, name_class);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxonDTO {
    pub tax_id: i64,
    pub tax_name: String,
    pub unique_name: Option<String>,
    pub name_class: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExtendedTaxonDTO {
    // This is the same columns from the names.dmp
    pub tax_id: i64,
    pub tax_name: String,
    pub unique_name: Option<String>,
    pub name_class: String,

    // This columns exists on nodes.dmp
    pub parent: i64,
    pub rank: String,

    // This columns exists on division.dmp
    pub division: String,
}

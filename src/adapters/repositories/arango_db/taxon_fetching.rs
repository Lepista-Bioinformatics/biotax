use std::collections::HashMap;

use crate::domain::{
    dtos::taxon::ExtendedTaxonDTO,
    entities::taxon_fetching::{GetResponseKind, TaxonFetching},
    utils::errors::MappedErrors,
};
use async_trait::async_trait;
use log::debug;
use rust_arango::{ClientError, Database};
use serde_json::value::to_value;
use shaku::Component;

#[derive(Component)]
#[shaku(interface = TaxonFetching)]
pub struct TaxonFetchingArangodbRepository {
    pub db: Database,
    pub collection: &'static str,
}

#[async_trait]
impl TaxonFetching for TaxonFetchingArangodbRepository {
    // This method filters the the database records and return an array of
    // taxon methods.
    async fn get(&self, tax_id: i64) -> Result<GetResponseKind, MappedErrors> {
        debug!("Validating taxid.");
        if !tax_id.ge(&1) {
            return Err(MappedErrors::new(
                "Taxid should be greater than 1",
                None,
                None,
            ));
        };

        debug!("Building query parameters.");
        let mut vars = HashMap::new();

        vars.insert(
            "@collection".to_string(),
            to_value(self.collection.to_string()).unwrap(),
        );

        vars.insert("tax_id".to_string(), to_value(tax_id).unwrap());

        debug!("Fetching taxid metadata.");
        let resp: Result<Vec<ExtendedTaxonDTO>, ClientError> = self
            .db
            .aql_bind_vars(
                r#"FOR n IN @@collection FILTER n.tax_id == @tax_id RETURN n"#,
                vars,
            )
            .await;

        if resp.is_err() {
            return Err(MappedErrors::new(
                "Unexpected error detected on fetch taxids from database.",
                None,
                None,
            ));
        }

        let records = resp.unwrap().to_vec();

        debug!("Successfully recovered metadata from database.");

        if records.len() == 0 {
            return Ok(GetResponseKind::NotFound(tax_id));
        }

        Ok(GetResponseKind::Found(records))
    }
}

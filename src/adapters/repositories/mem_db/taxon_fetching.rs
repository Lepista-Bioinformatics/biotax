use crate::{
    domain::{
        entities::taxon_fetching::{GetResponseKind, TaxonFetching},
        utils::errors::MappedErrors,
    },
    use_cases::load_source_dump_database::TaxonDatabase,
};
use async_trait::async_trait;
use shaku::Component;

#[derive(Clone, Component)]
#[shaku(interface = TaxonFetching)]
pub struct TaxonFetchingMemDbRepository {
    #[shaku(default)]
    pub db: TaxonDatabase,
}

#[async_trait]
impl TaxonFetching for TaxonFetchingMemDbRepository {
    // This method filters the the database records and return an array of
    // taxon methods.
    async fn get(&self, tax_id: i64) -> Result<GetResponseKind, MappedErrors> {
        if !tax_id.ge(&1) {
            return Err(MappedErrors::new(
                "Taxid should be greater than 1",
                None,
                None,
            ));
        };

        let records = self.db.get(&tax_id.to_string());

        if records.is_none() {
            return Ok(GetResponseKind::NotFound(tax_id));
        }

        Ok(GetResponseKind::Found(records.unwrap().to_vec()))
    }
}

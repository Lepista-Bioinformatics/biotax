use crate::domain::{
    dtos::taxon::TaxonDTO, entities::taxon_registration::TaxonRegistration,
    utils::errors::MappedErrors,
};
use async_trait::async_trait;
use pickledb::PickleDb;

pub struct TaxonRegistrationKvDbRepository;

#[async_trait]
impl TaxonRegistration for TaxonRegistrationKvDbRepository {
    async fn get_or_create(
        &self,
        db: &mut PickleDb,
        taxon: TaxonDTO,
    ) -> Result<TaxonDTO, MappedErrors> {
        let taxon_exists = db.exists("key");

        if !taxon_exists {
            db.set(&taxon.tax_id.to_string(), &taxon).unwrap();
        }

        Ok(taxon)
    }
}

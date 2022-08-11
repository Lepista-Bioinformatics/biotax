use crate::domain::{
    dtos::taxon::TaxonDTO, entities::taxon_fetching::TaxonFetching,
    utils::errors::MappedErrors,
};
use async_trait::async_trait;
use pickledb::PickleDb;

pub struct TaxonFetchingKvDbRepository;

//#[async_trait]
//impl TaxonFetching for TaxonFetchingKvDbRepository {
//    async fn get(
//        &self,
//        db: &mut PickleDb,
//        taxon: TaxonDTO,
//    ) -> Result<TaxonDTO, MappedErrors> {
//        let taxon_exists = db.exists("key");
//
//        if !taxon_exists {
//            db.set(&taxon.tax_id.to_string(), &taxon).unwrap();
//        }
//
//        Ok(taxon)
//    }
//}

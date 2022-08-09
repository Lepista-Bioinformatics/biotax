use crate::{
    domain::{
        dtos::taxon::TaxonDTO, entities::taxon_fetching::TaxonFetching,
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
    async fn get(&self, tax_id: i64) -> Result<Vec<TaxonDTO>, MappedErrors> {
        if !tax_id.ge(&1) {
            return Err(MappedErrors::new(
                "Taxid should be greater than 1",
                None,
                None,
            ));
        };

        let records = self.db.get(&tax_id.to_string());

        if records.is_none() {
            return Ok([].to_vec());
        }

        Ok(records.unwrap().to_vec())
    }
}

// ? --------------------------------------------------------------------------
// ? Tests
// ? --------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::adapters::repositories::mem_db::taxon_fetching::TaxonFetchingMemDbRepository;
    use crate::domain::entities::taxon_fetching::TaxonFetching;
    use crate::use_cases::load_source_dump_database::load_source_dump_database;
    use futures::executor::block_on;

    #[test]
    fn taxon_list_test() {
        // Load mock database
        let db = load_source_dump_database(
            "/home/samuel-elias/study-projects/rust/biotax/src/assets/names-tab-200.dmp"
        );

        assert_eq!(db.is_err(), false);

        let repo = TaxonFetchingMemDbRepository { db: db.unwrap() };

        match block_on(repo.get(106)) {
            Ok(res) => assert_eq!(res.len(), 0),
            Err(err) => assert_eq!(err.to_string(), ""),
        };

        match block_on(repo.get(50)) {
            Ok(res) => assert_eq!(res.len(), 6),
            Err(err) => assert_eq!(err.to_string(), ""),
        };

        println!("{:?}", block_on(repo.get(50)));
    }
}

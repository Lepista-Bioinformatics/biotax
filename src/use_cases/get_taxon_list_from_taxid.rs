use crate::domain::{
    dtos::taxon::TaxonDTO, entities::taxon_fetching::TaxonFetching,
    utils::errors::MappedErrors,
};

#[allow(dead_code)]
pub async fn get_taxon_list_from_taxid(
    tax_id: i64,
    taxon_fetching_repo: Box<&dyn TaxonFetching>,
) -> Result<Vec<TaxonDTO>, MappedErrors> {
    // Simple fetch records from taxa repo
    taxon_fetching_repo.get(tax_id).await
}

// ? --------------------------------------------------------------------------
// ? Tests
// ? --------------------------------------------------------------------------

#[cfg(test)]
mod test {

    use crate::use_cases::load_source_dump_database::load_source_dump_database;

    #[test]
    fn get_taxon_list_from_taxid_test() {
        // Load mock database
        let db = load_source_dump_database("assets/names-tab-200.dmp");

        assert_eq!(db.is_err(), false);

        // Build the repository object
        //let repo = TaxonFetchingMemBdRepository { db: db.unwrap() };

        // Fetch results
        /* let future = get_taxon_list_from_taxid(51, repo.clone());
        let response = block_on(future);

        assert_eq!(response.is_err(), false);
        assert_eq!(response.unwrap().len(), 2); */
    }
}

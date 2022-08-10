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

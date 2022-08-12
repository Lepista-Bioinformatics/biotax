use crate::domain::{
    entities::taxon_fetching::{GetResponseKind, TaxonFetching},
    utils::errors::MappedErrors,
};

pub async fn get_taxon_list_from_taxid(
    tax_id: i64,
    taxon_fetching_repo: Box<&dyn TaxonFetching>,
) -> Result<GetResponseKind, MappedErrors> {
    // Simple fetch records from taxa repo
    taxon_fetching_repo.get(tax_id).await
}

use crate::domain::{dtos::taxon::TaxonDTO, utils::errors::MappedErrors};
use async_trait::async_trait;

// These trait defines two optional methods, `get` and `list`. Both methods
// should be further implemented.
#[async_trait]
pub trait TaxonFetching: Send + Sync {
    async fn get(&self, tax_id: i64) -> Result<Vec<TaxonDTO>, MappedErrors>;
}

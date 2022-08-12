use std::fmt::Debug;

use crate::domain::dtos::taxon::ExtendedTaxonDTO;
use crate::domain::utils::errors::MappedErrors;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use shaku::Interface;

#[derive(Debug, Serialize, Deserialize)]
pub enum GetResponseKind {
    Found(Vec<ExtendedTaxonDTO>),
    NotFound(i64),
}

// These trait defines two optional methods, `get` and `list`. Both methods
// should be further implemented.
#[async_trait]
pub trait TaxonFetching: Interface + Send + Sync {
    async fn get(&self, tax_id: i64) -> Result<GetResponseKind, MappedErrors>;
}

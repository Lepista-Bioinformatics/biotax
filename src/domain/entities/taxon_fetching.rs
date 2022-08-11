use std::fmt::Result as FmResult;
use std::fmt::{Debug, Display, Formatter};

use crate::domain::dtos::taxon::ExtendedTaxonDTO;
use crate::domain::utils::errors::MappedErrors;
use async_trait::async_trait;
use shaku::Interface;

// These trait defines two optional methods, `get` and `list`. Both methods
// should be further implemented.
#[async_trait]
pub trait TaxonFetching: Interface + Send + Sync {
    async fn get(
        &self,
        tax_id: i64,
    ) -> Result<Vec<ExtendedTaxonDTO>, MappedErrors>;
}

impl Display for dyn TaxonFetching {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmResult {
        write!(f, "{}", self)
    }
}

impl Debug for dyn TaxonFetching {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmResult {
        write!(f, "{}", self)
    }
}

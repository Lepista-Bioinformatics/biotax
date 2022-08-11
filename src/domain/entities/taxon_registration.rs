use std::fmt::Result as FmResult;
use std::fmt::{Debug, Display, Formatter};

use crate::domain::{dtos::taxon::TaxonDTO, utils::errors::MappedErrors};
use async_trait::async_trait;
use pickledb::PickleDb;
use shaku::Interface;

// These trait defines two optional methods, `get` and `list`. Both methods
// should be further implemented.
#[async_trait]
pub trait TaxonRegistration: Interface + Send + Sync {
    async fn get_or_create(
        &self,
        db: &mut PickleDb,
        taxon: TaxonDTO,
    ) -> Result<TaxonDTO, MappedErrors>;
}

impl Display for dyn TaxonRegistration {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmResult {
        write!(f, "{}", self)
    }
}

impl Debug for dyn TaxonRegistration {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmResult {
        write!(f, "{}", self)
    }
}

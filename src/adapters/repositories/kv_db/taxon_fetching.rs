use crate::domain::{
    dtos::taxon::ExtendedTaxonDTO,
    entities::taxon_fetching::{GetResponseKind, TaxonFetching},
    utils::errors::MappedErrors,
};
use async_trait::async_trait;
use pickledb::PickleDb;
use shaku::Component;

pub struct CustomPickleDb {
    pub conn: PickleDb,
}

impl Default for CustomPickleDb {
    #[allow(unconditional_recursion)]
    fn default() -> Self {
        Self::default()
    }
}

#[derive(Component)]
#[shaku(interface = TaxonFetching)]
pub struct TaxonFetchingKvDbRepository {
    #[shaku(default)]
    pub db: CustomPickleDb,
}

#[async_trait]
impl TaxonFetching for TaxonFetchingKvDbRepository {
    async fn get(&self, tax_id: i64) -> Result<GetResponseKind, MappedErrors> {
        let taxon = self.db.conn.exists(&tax_id.to_string());

        if !taxon {
            return Ok(GetResponseKind::NotFound(tax_id));
        }

        Ok(GetResponseKind::Found(
            self.db
                .conn
                .get::<Vec<ExtendedTaxonDTO>>(&tax_id.to_string())
                .unwrap(),
        ))
    }
}

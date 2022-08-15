use crate::domain::{
    dtos::taxon::ExtendedTaxonDTO,
    entities::taxon_registration::TaxonRegistration,
    utils::errors::MappedErrors,
};
use async_trait::async_trait;
use log::warn;
use reqwest::Client;
use shaku::Component;

#[derive(Clone, Component)]
#[shaku(interface = TaxonRegistration)]
pub struct TaxonRegistrationArangoDbRepository {
    //pub reqwest_client: Client,
    pub service_url: String,
    pub username: String,
    pub password: Option<String>,
}

#[async_trait]
impl TaxonRegistration for TaxonRegistrationArangoDbRepository {
    async fn get_or_create(
        &self,
        tax_id: String,
        taxon: Vec<ExtendedTaxonDTO>,
    ) -> Result<String, MappedErrors> {
        let request = Client::new()
            .post(&self.service_url)
            .basic_auth(&self.username, self.password.as_ref())
            .json(&taxon)
            .send()
            .await;

        let response = request.unwrap();

        if response.status().is_client_error() {
            warn!("{}", response.text().await.unwrap());

            return Err(MappedErrors::new(
                "Unexpected error detected on persist taxon.",
                None,
                None,
            ));
        }

        Ok(tax_id)
    }
}

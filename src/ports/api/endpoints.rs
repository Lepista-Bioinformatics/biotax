extern crate biotax;

use actix_web::{
    error::ErrorBadRequest, get, web, HttpResponse, Responder, Result,
};

use biotax::{
    domain::entities::taxon_fetching::{GetResponseKind, TaxonFetching},
    use_cases::get_taxon_list_from_taxid::get_taxon_list_from_taxid,
};

use log::{debug, warn};
use shaku_actix::Inject;

use crate::modules::TaxonFetchingModule;

#[get("/health")]
pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("success".to_string()))
}

#[get("/taxids/{taxid}")]
async fn resolve_taxid(
    taxid: web::Path<i64>,
    repo: Inject<TaxonFetchingModule, dyn TaxonFetching>,
) -> impl Responder {
    debug!("Fetching taxid metadata information.");
    let result =
        get_taxon_list_from_taxid(taxid.into_inner(), Box::from(&*repo)).await;

    if result.is_err() {
        warn!("Fetching taxid metadata information.");
        return Err(ErrorBadRequest(result.unwrap_err()));
    }

    let get_response: GetResponseKind = result.unwrap().into();

    Ok(web::Json(get_response))
}

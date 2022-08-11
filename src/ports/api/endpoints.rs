extern crate biotax;

use actix_web::{
    error::ErrorBadRequest, get, web, HttpResponse, Responder, Result,
};

use biotax::{
    domain::entities::taxon_fetching::TaxonFetching,
    use_cases::get_taxon_list_from_taxid::get_taxon_list_from_taxid,
};

use crate::modules::TaxonFetchingModule;

use shaku_actix::Inject;

#[get("/health")]
pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("success".to_string()))
}

#[get("/taxids/{taxid}")]
async fn resolve_taxid(
    taxid: web::Path<i64>,
    repo: Inject<TaxonFetchingModule, dyn TaxonFetching>,
) -> Result<impl Responder> {
    let response =
        get_taxon_list_from_taxid(taxid.into_inner(), Box::from(&*repo)).await;

    if response.is_err() {
        return Err(ErrorBadRequest(response.unwrap_err()));
    }

    Ok(web::Json(response.unwrap()))
}

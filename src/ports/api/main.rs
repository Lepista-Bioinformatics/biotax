use actix_web::{error::ErrorBadRequest, get, web, Responder, Result};

use crate::{
    domain::entities::taxon_fetching::TaxonFetching,
    ports::api::modules::TaxonFetchingModule,
    use_cases::get_taxon_list_from_taxid::get_taxon_list_from_taxid,
};
use shaku_actix::Inject;

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

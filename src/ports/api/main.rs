use actix_web::{error::ErrorBadRequest, get, web, Responder, Result};

use crate::{
    domain::entities::taxon_fetching::TaxonFetching,
    use_cases::get_taxon_list_from_taxid::get_taxon_list_from_taxid,
};

#[get("/{taxid}")]
async fn resolve_taxid(
    path: web::Path<i64>,
    taxon_fetching_repo: web::Data<Box<dyn TaxonFetching>>,
) -> Result<impl Responder> {
    // TODO: Move all dependencies to dedicated injector. See the example of
    // TODO: the below URL:
    // TODO: https://www.vultr.com/docs/building-rest-apis-in-rust-with-actix-web/

    // Try to fetch taxid
    let taxid = path.into_inner();
    let response =
        get_taxon_list_from_taxid(taxid, &*taxon_fetching_repo.into_inner())
            .await;

    if response.is_err() {
        return Err(ErrorBadRequest(response.unwrap_err()));
    }

    Ok(web::Json(response.unwrap()))
}

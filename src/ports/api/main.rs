use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    domain::entities::taxon_fetching::TaxonFetching,
    use_cases::get_taxon_list_from_taxid::get_taxon_list_from_taxid,
};

#[get("/{taxid}")]
async fn resolve_taxid(
    taxid: web::Path<i64>,
    context: web::Data<dyn TaxonFetching>,
) -> impl Responder {
    //Result<impl Responder>
    // TODO: Move all dependencies to dedicated injector. See the example of
    // TODO: the below URL:
    // TODO: https://www.vultr.com/docs/building-rest-apis-in-rust-with-actix-web/

    //println!("{:?}", taxid);
    println!("{:?}", context);

    let repo = context.into_inner();
    //println!("{}", repo.expect("Error"));

    // Try to fetch taxid
    let response =
        get_taxon_list_from_taxid(taxid.into_inner(), Box::from(&*repo)).await;

    println!("{:?}", response);

    //if response.is_err() {
    //    return Err(ErrorBadRequest(response.unwrap_err()));
    //}

    //Ok(web::Json(response.unwrap()))
    HttpResponse::Ok()
}

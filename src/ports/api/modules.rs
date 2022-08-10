use crate::adapters::repositories::mem_db::taxon_fetching::TaxonFetchingMemDbRepository;
use shaku::module;

module! {
    pub TaxonFetchingModule {
        components = [TaxonFetchingMemDbRepository],
        providers = []
    }
}
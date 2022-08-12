extern crate biotax;

use biotax::adapters::repositories::kv_db::taxon_fetching::TaxonFetchingKvDbRepository;
use shaku::module;

module! {
    pub TaxonFetchingModule {
        components = [TaxonFetchingKvDbRepository],
        providers = []
    }
}

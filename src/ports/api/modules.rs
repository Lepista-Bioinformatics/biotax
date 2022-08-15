extern crate biotax;

use biotax::adapters::repositories::arango_db::taxon_fetching::TaxonFetchingArangodbRepository;
use shaku::module;

module! {
    pub TaxonFetchingModule {
        components = [TaxonFetchingArangodbRepository],
        providers = []
    }
}

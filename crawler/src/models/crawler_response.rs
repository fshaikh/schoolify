use std::fmt;
use crate::models::catchment_area::CatchmentArea;
use crate::models::mapping::Mapping;
/// Represents a crawler response. All region crawlers must return response confirming to this
/// data structure
use crate::models::school::School;

pub struct CrawlerResponse {
    pub schools: Vec<School>,
    pub catchmentareas: Vec<CatchmentArea>,
    pub mapping: Mapping,
}

impl fmt::Debug for CrawlerResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CrawlerResponse")
            .field("mapping", &self.mapping)
            .finish()
    }
}


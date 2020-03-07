/// Represents a crawler response. All region crawlers must return response confirming to this
/// data structure
use crate::models::school::{School};
use crate::models::catchment_area::CatchmentArea;
use crate::models::mapping::Mapping;

pub struct CrawlerResponse {
    pub schools: Vec<School>,
    pub catchmentareas: Vec<CatchmentArea>,
    pub mapping: Mapping,
}

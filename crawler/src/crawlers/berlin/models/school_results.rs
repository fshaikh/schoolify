use crate::crawlers::berlin::models::details_result::DetailsResult;
use crate::crawlers::berlin::models::root_result::RootResult;

pub struct SchoolResult {
    pub root_result: RootResult,
    pub details_result: DetailsResult,
}

pub struct SchoolResults {
    pub results: Vec<SchoolResult>,
}

impl SchoolResults {
    pub fn new() -> SchoolResults {
        SchoolResults {
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: SchoolResult) {
        self.results.push(result);
    }
}

#[derive(Debug)]
pub struct RootResult {
    pub id: String,
    pub details_url: String,
    pub school_name: String,
    pub sub_district: String,
    pub district: String,
    pub school_type: String,
}

pub struct RootResults {
    pub results: Vec<RootResult>,
}

impl RootResults {
    pub fn new() -> RootResults {
        RootResults {
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: RootResult) {
        self.results.push(result)
    }

    pub fn get_results(&self, start: usize, end: usize) -> &[RootResult] {
        &self.results[start..end]
    }

    // pub fn count(&self) -> usize {
    //     self.results.len()
    // }

    // pub fn get_ids(&self) -> Vec<String> {
    //     let ids: Vec<String> = Vec::new();
    //     for result in self.results {
    //         ids.push(result.id);
    //     }
    //     ids
    // }
}

// for let root_result in root_results {
// root_result
// }
// TODO: Implement an iterator instead of exposing inner results vector
// impl Iterator for RootResults {
//     type Item = RootResult;
//     fn next(&mut self) -> Option<RootResult> {
//         //results.get
//         let value = self.results.next();
//     }
// }

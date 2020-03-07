// /// A school can belong to only one catchment area. Though not each school will be part of a catchment area.
// ///  This is because only public schools (or government-funded schools) are allotted to catchment area.
// /// Stores mapping of catchment area to schools
use std::collections::HashMap;

pub struct Mapping {
    catchmentarea_school_mapping: HashMap<String, Vec<String>>, // pub catchment_area : crate::Models::CatchmentArea::CatchmentArea,
                                                                // pub schools: Vec<crate::Models::School::School>
}

// impl Mapping {
//     /// Constructs a new Mapping struct
//     fn construct() -> crate::Models::Mapping::Mapping {
//         Mapping {
//             catchmentarea_school_mapping: HashMap::new(),
//         }
//     }

//     /// Adds schools to a particular catchment area
//     fn add(&self, catchmentarea_id: String, schools: Vec<String>) {}

//     /// Adds a school to a particular catchment area
//     fn add_school(&self, catchmentarea_id: String, school_id: String) {}

//     /// Gets schools for a catchment area
//     fn get(&self, catchmentarea_id: String) -> Vec<String> {
//         return vec!["".to_string()];
//     }

//     /// Deletes a catchment area mapping
//     fn delete(&self, catchmentarea_id: String) {}
// }

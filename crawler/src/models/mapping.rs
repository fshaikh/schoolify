// /// A school can belong to only one catchment area. Though not each school will be part of a catchment area.
// ///  This is because only public schools (or government-funded schools) are allotted to catchment area.
// /// Stores mapping of catchment area to schools
use crate::models::school::Schools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Mapping {
    catchmentarea_school_mapping: HashMap<String, Schools>, // pub catchment_area : crate::Models::CatchmentArea::CatchmentArea,
                                                            // pub schools: Vec<crate::Models::School::School>
}

impl Mapping {
    /// Constructs a new Mapping struct
    pub fn new() -> Mapping {
        Mapping {
            catchmentarea_school_mapping: HashMap::new(),
        }
    }

    /// Adds a school to a particular catchment area
    pub fn add_school(&mut self, catchmentarea_id: String, school_id: String) {
        let s = self.catchmentarea_school_mapping.get_mut(&catchmentarea_id);
        match s {
            None => {
                let mut schools = Schools::new();
                schools.add_school(school_id);
                self.catchmentarea_school_mapping
                    .insert(catchmentarea_id, schools);
            }
            Some(existing_schools) => existing_schools.add_school(school_id),
        }
    }

    pub fn get_schools(&self, catchmentarea_id: String) -> Option<&Schools> {
        self.catchmentarea_school_mapping.get(&catchmentarea_id)
    }
}

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::school::Schools;

    #[test]
    fn it_should_insert() {
        let mut mapping = Mapping::new();
        mapping.add_school("1".to_string(), "1".to_string());
        mapping.add_school("1".to_string(), "11".to_string());
        mapping.add_school("2".to_string(), "2".to_string());

        let schools_option = mapping.get_schools("1".to_string());
        match schools_option {
            None => assert_eq!(true, false),
            Some(schools) => {
                let raw_schools = schools.get_schools();
                assert_eq!(raw_schools.iter().count(), 2);
                assert_eq!(raw_schools[0], "1");
                assert_eq!(raw_schools[1], "11");
            }
        }

        let schools_option2 = mapping.get_schools("2".to_string());
        match schools_option2 {
            None => assert_eq!(true, false),
            Some(schools) => {
                let raw_schools = schools.get_schools();
                assert_eq!(raw_schools.iter().count(), 1);
                assert_eq!(raw_schools[0], "2");
            }
        }

        let schools_option3 = mapping.get_schools("3".to_string());
        match schools_option3 {
            None => assert_eq!(true, true),
            _ => assert_eq!(true, false),
        }
    }
}
//#endregion Tests

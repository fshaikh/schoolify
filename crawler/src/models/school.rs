use crate::models::location::Location;
/// Data structure representing School domain object
///
use crate::models::object_base::ObjectBase;

#[derive(Debug)]
pub enum FundingType {
    Public,
    Private,
}

#[derive(Debug)]
pub enum SchoolType {
    Kindergarten,
    Primary,
    Secondary,
}

#[derive(Debug)]
pub struct School {
    pub meta: ObjectBase,
    /// School Id provided by the data source
    pub school_id: String,
    /// Name of the school
    pub name: String,
    /// Type of school
    pub school_type: SchoolType,
    /// Primary phone number
    pub primary_phone_number: String,
    /// Email address
    pub email: String,
    /// Url of the school website
    pub url: String,
    /// Names of key contact persons
    pub contact_persons: Vec<String>,
    /// Languages supported by school
    pub languages: Vec<String>,
    /// True – if school has multiple languages else false.
    ///  Computed property from Languages property
    pub is_bilingual: bool,
    /// Type of school funding
    pub funding_type: FundingType,
    /// Address of school stored as a single unit. We can see later if we need to normalize it
    pub address: String,
    /// District the school is in
    pub district: String,
    /// School fees
    pub fees: String,
    /// Lat/lon for the school based on the address.
    pub location: Option<Location>,
    /// Catchment Area Id of the school. If school does not belong to any catchment area, this will be blank.
    ///  This is useful to fetch schools based on location proximity and not just catchment area.
    pub catchmentarea_id: Option<String>,
}

impl School {
    pub fn get_id(&self) -> &String {
        &self.meta.id
    }

    pub fn get_catchmentarea_id(&self) -> Option<&String> {
        self.catchmentarea_id.as_ref()
    }
}

#[derive(Debug)]
pub struct Schools {
    schools: Vec<String>,
}

impl Schools {
    pub fn new() -> Schools {
        Schools {
            schools: Vec::new(),
        }
    }

    pub fn add_school(&mut self, id: String) {
        self.schools.push(id);
    }

    pub fn get_schools(&self) -> &Vec<String> {
        &self.schools
    }
}

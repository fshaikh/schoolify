/// Model representating the details result of a school crawling
pub struct DetailsResult {
    pub address: Option<String>,
    pub tel: Option<String>,
    pub fax: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
    pub primary_contact: Option<String>,
    pub languages: Option<Vec<String>>,
}

impl DetailsResult {
    pub fn get_address(&self) -> String {
        return self.get_string_value(&self.address);
    }
    pub fn get_tel(&self) -> String {
        return self.get_string_value(&self.tel);
    }
    pub fn get_fax(&self) -> String {
        return self.get_string_value(&self.fax);
    }
    pub fn get_email(&self) -> String {
        return self.get_string_value(&self.email);
    }
    pub fn get_url(&self) -> String {
        return self.get_string_value(&self.url);
    }
    pub fn get_primary_contact(&self) -> String {
        return self.get_string_value(&self.primary_contact);
    }

    fn get_string_value(&self, value: &Option<String>) -> String {
        return match value {
            Some(val) => val.to_string(),
            None => "".to_string(),
        };
    }
    pub fn format(&self) -> String {
        return format!(
            "Address: {},
                        Tel: {},
                        Fax: {},
                        Email: {},
                        Url: {},
                        Primary Contact: {},
                        languages: {:?}
                        ",
            self.get_address(),
            self.get_tel(),
            self.get_fax(),
            self.get_email(),
            self.get_url(),
            self.get_primary_contact(),
            self.languages
        );
    }
}

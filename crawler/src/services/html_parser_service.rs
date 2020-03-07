use scraper::{element_ref::ElementRef, Html, Selector};

pub mod html_names {
    pub const HREF: &str = "href";
    pub const A: &str = "a";
    pub const SPAN: &str = "span";
}

pub struct HtmlParserService {
    document: Html,
}

impl<'a> HtmlParserService {
    pub fn new(html: &String) -> HtmlParserService {
        let document: Html = Html::parse_document(html);
        HtmlParserService { document }
    }

    // pub fn get(&self, selector: &String) {
    //     let selector = Selector::parse(selector).unwrap();
    //     for element in self.document.select(&selector) {
    //         // let selector1 = Selector::parse("td a").unwrap();
    //         // for desc in element.select(&selector1) {
    //         //     println!("{:?}",desc.value().name());
    //         // }
    //         // element.value() returns an element of type Element
    //         let attributes = element.value().attrs();
    //         for attr in attributes {
    //             println!("{:?}", attr)
    //         }
    //         // gives all the attributes of 'a'
    //         // ("style", "font-weight:bold;")
    //         // ("href", "https://www.berlin.de/sen/bildung/schule/berliner-schulen/schulverzeichnis/Schulportrait.aspx?IDSchulzweig=%2020292")
    //         // ("id", "DataListSchulen_HLinkSchulNr_0")

    //         // descendants
    //         println!("{:?}", element.text().collect::<Vec<_>>());
    //         // Gives text value
    //         // ["09P05"]
    //     }
    // }

    pub fn select_from_root(&self, selector: &str) -> Option<Vec<ElementRef>> {
        let selector: Selector = HtmlParserService::create_selector(selector);
        // return HtmlParserService::select_core(self.document.select(&selector));
        let mut elements = Vec::new();

        for element in self.document.select(&selector) {
            // let html_element = element; // html_element : : scraper::node::Element
            // let u: u32 = element;
            elements.push(element);
        }
        if elements.len() == 0 {
            return None;
        }
        return Some(elements);
    }

    pub fn select_from_element(
        &'a self,
        element: &'a ElementRef,
        selector: &str,
    ) -> Option<Vec<ElementRef>> {
        let selector: Selector = HtmlParserService::create_selector(selector);
        // return HtmlParserService::select_core(element.select(&selector));
        let mut elements = Vec::new();
        // let i: u32 = element.select(&selector);

        for selected_element in element.select(&selector) {
            // let html_element = element.value(); // html_element : : scraper::node::Element
            elements.push(selected_element);
        }
        if elements.len() == 0 {
            return None;
        }
        return Some(elements);
    }

    pub fn get_element_text(&self, element: &ElementRef) -> Option<String> {
        let text = element.text().collect::<Vec<_>>();
        if text.len() == 0 {
            return None;
        }
        return Some(text[0].to_string());
    }

    pub fn get_attr(&self, element: &ElementRef, attr_name: &str) -> Option<String> {
        let attr_value_option = element.value().attr(attr_name);
        match attr_value_option {
            Some(value) => return Some(String::from(value)),
            None => return None,
        };
    }

    fn create_selector(selector: &str) -> Selector {
        return Selector::parse(selector).unwrap();
    }

    // fn select_core(select: &scraper::html::Select) -> Option<Vec<&scraper::node::Element>> {
    //     let elements = Vec::new();

    //     for element in select {
    //         let html_element = element.value(); // html_element : : scraper::node::Element
    //         elements.push(html_element);
    //     }
    //     if elements.len() == 0 {
    //         return None;
    //     }
    //     return Some(elements);
    // }
}

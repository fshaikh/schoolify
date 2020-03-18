use crate::crawlers::berlin::models::details_result::DetailsResult;
use crate::crawlers::berlin::models::root_result::{RootResult, RootResults};
use crate::models::error::Error;
use crate::services::html_parser_service::html_names::{A, HREF, SPAN};
use crate::services::html_parser_service::HtmlParserService;

//#region CONST Members
const ROOT_SELECTOR: &str = "table#DataListSchulen td";
const DETAILS_MAIN_SELECTOR: &str = ".portrait_rechts";
const SCHOOLS_EXPECTED_COUNT: usize = 1396;
const SCHOOLS_ROOT_SPAN_COUNT: usize = 4;
const ID: &str = "id";
const SCHOOL_ID_NAME: &str = "SchulName";
const SCHOOL_ID_TYPE: &str = "Schulart";
const SCHOOL_ID_SUBDISTRICT: &str = "Ortsteil";
const SCHOOL_ID_DISTRICT: &str = "Bezirk";
const SCHOOL_ADDRESS_STREET_ID: &str = "#ContentPlaceHolderMenuListe_lblStrasse";
const SCHOOL_ADDRESS_PINCODE_ID: &str = "#ContentPlaceHolderMenuListe_lblOrt";
const SCHOOL_TEL_ID: &str = "#ContentPlaceHolderMenuListe_lblTelefon";
const SCHOOL_FAX_ID: &str = "#ContentPlaceHolderMenuListe_lblFax";
const SCHOOL_EMAIL_ID: &str = "#ContentPlaceHolderMenuListe_HLinkEMail";
const SCHOOL_WEBURL_ID: &str = "#ContentPlaceHolderMenuListe_HLinkWeb";
const SCHOOL_PRIMARYCONTACT_ID: &str = "#ContentPlaceHolderMenuListe_lblLeitung";
const SCHOOL_LANGUAGES_ID: &str = "#ContentPlaceHolderMenuListe_lblSprachen";
//#endregion CONST Members

//#region public API
pub fn get_school_root_results(root_html: &String) -> Result<RootResults, Error> {
    let parser = get_html_parser(root_html)?;

    println!("html_parser_adaptor::get_school_root_results - Constructed Html Parser");
    // first get all the tds
    let elements_option = parser.select_from_root(ROOT_SELECTOR);
    if elements_option.is_none() {
        return Err(Error {
            message: "Unable to parse Root for schools".to_string(),
        });
    }

    let elements = elements_option.unwrap();
    println!(
        "HtmlParserAdaptor::get_school_root_results: Found: {} Schools, Expected Schools: {}",
        elements.len(),
        SCHOOLS_EXPECTED_COUNT
    );

    // for each element "td" get the attributes and text descendant
    let mut index = 0;
    let mut root_results: RootResults = RootResults::new();
    for element in elements {
        // hierarchy is :
        // <td>
        //    <a id="DataListSchulen_HLinkSchulNr_557" href="Schulportrait.aspx?IDSchulzweig= 20366">03G04</a>
        //    <span id="DataListSchulen_lblSchulName_557" >Grundschule an der Marie</span><br>
        //    <span id="DataListSchulen_lblSchulart_557">Grundschule</span><br>
        //    <span id="DataListSchulen_lblOrtsteil_557">Prenzlauer Berg</span>
        //    <span id="DataListSchulen_lblBezirk_557">Pankow</span>
        // </td>

        // Parse <a> element
        let school_id_details_tuple_option = parse_root_a_element(&parser, &element, index);
        if school_id_details_tuple_option.is_none() {
            println!(
                "HtmlParserAdaptor::get_school_root_results: <a> tag not found for index: {}",
                index
            );
        }
        let school_id_details_tuple = school_id_details_tuple_option.unwrap();

        // Parse all <span> elements
        let root_school_info_option: Option<RootSchoolInfo> =
            parse_root_span_elements(&parser, &element, &school_id_details_tuple.0, index);
        if root_school_info_option.is_none() {
            println!(
                "HtmlParserAdaptor::get_school_root_results: <span> tags not found for index: {}",
                index
            );
        }
        let root_school_info = root_school_info_option.unwrap();

        let root_result = RootResult {
            id: school_id_details_tuple.0,
            details_url: school_id_details_tuple.1,
            school_name: root_school_info.name,
            school_type: root_school_info.school_type,
            sub_district: root_school_info.sub_district,
            district: root_school_info.district,
        };
        println!("Root school: {:?}", root_result);
        root_results.add_result(root_result);
        index = index + 1;
    }

    return Ok(root_results);
}

///Parses the school details from the provided HTML
///
pub fn get_school_details_result(
    _root_result: &RootResult,
    details_html: &String,
) -> Result<DetailsResult, Error> {
    let parser = get_html_parser(details_html)?;

    println!("html_parser_adaptor::get_school_details_result - Constructed Html Parser");
    let details_elements_option = parser.select_from_root(DETAILS_MAIN_SELECTOR);
    if details_elements_option.is_none() {
        return Err(Error {
            message: "Unable to parse Main Details for schools".to_string(),
        });
    }
    let elements = details_elements_option.unwrap();
    let details_main_element = elements[0];

    // parse basic details elements
    // parse address
    // parse tel
    // parse fax
    // parse email
    // parse web
    // parse main cntact
    // parse languages
    //
    return Ok(DetailsResult {
        address: get_address(&parser, &details_main_element),
        tel: get_tel(&parser, &details_main_element),
        fax: get_fax(&parser, &details_main_element),
        email: get_email(&parser, &details_main_element),
        url: get_weburl(&parser, &details_main_element),
        primary_contact: get_primary_contact(&parser, &details_main_element),
        languages: get_languages(&parser, &details_main_element),
        location: None,
    });
}

//#endregion public API

//#region Private Area
fn get_html_parser(html: &String) -> Result<HtmlParserService, Error> {
    HtmlParserService::new(html).or_else(|err| {
        println!(
            "html_parser_adaptor::get_school_root_results - Unable to crate Html Parser: {}",
            err.message
        );
        Err(err)
    })
}
fn parse_root_a_element(
    parser: &HtmlParserService,
    element: &scraper::element_ref::ElementRef,
    index: u32,
) -> Option<(String, String)> {
    let a_element_option = parser.select_from_element(element, A);
    if a_element_option.is_none() {
        return None;
    }
    let mut details_url: String = "".to_string();
    let mut school_id: String = "".to_string();
    let a_element = a_element_option.unwrap()[0];

    // parse href attribute which contains school details url
    let href_attr_option = parser.get_attr(&a_element, HREF);
    if href_attr_option.is_none() {
        println!(
            "HtmlParserAdaptor::parse_root_a_element: href atrribute not found for index: {}",
            index
        );
    } else {
        details_url = href_attr_option.unwrap();
    }

    // parse a text which contains school id
    let school_id_option = parser.get_element_text(&a_element);
    if school_id_option.is_none() {
        println!("HtmlParserAdaptor::parse_root_a_element: school id not found for index: {}. Details URL: {}", index, details_url);
    } else {
        school_id = school_id_option.unwrap();
    }
    return Some((school_id, details_url));
}

fn parse_root_span_elements(
    parser: &HtmlParserService,
    element: &scraper::element_ref::ElementRef,
    school_id: &String,
    index: u32,
) -> Option<RootSchoolInfo> {
    let span_elements_option = parser.select_from_element(element, SPAN);
    if span_elements_option.is_none() {
        println!("HtmlParserAdaptor::parse_root_span_elements : No <span> elements found under td for index: {}", index);
        return None;
    }
    // We should get 4 span elements. If we get less , we just print it and continue
    let span_elements = span_elements_option.unwrap();
    if span_elements.len() != SCHOOLS_ROOT_SPAN_COUNT {
        println!("HtmlParserAdaptor::parse_root_span_elements: Expected SPAN count: {}, Crawled count: {} for school_id: {} and index: {}", SCHOOLS_ROOT_SPAN_COUNT, span_elements.len(), school_id, index);
    }
    // How do we map each span to its intended value? There are 2 ways:
    // 1. Use index and assume the element order is the same
    // 2. Use id attribute of each span element and use that value to find the appropriate property. For eg:DataListSchulen_lblSchulName_557

    let mut root_school_info: RootSchoolInfo = Default::default();

    // We will go with 2nd approach as it is less error-prone though requires additional parsing which is ok since its in memory
    for span_element in span_elements {
        // get the id attribute
        let id_attr_option = parser.get_attr(&span_element, &ID);
        if id_attr_option.is_some() {
            let id_value = id_attr_option.unwrap();
            let text_option = parser.get_element_text(&span_element);
            if text_option.is_some() {
                let text = text_option.unwrap();
                if id_value.contains(SCHOOL_ID_NAME) {
                    root_school_info.name = text;
                } else if id_value.contains(SCHOOL_ID_TYPE) {
                    root_school_info.school_type = text;
                } else if id_value.contains(SCHOOL_ID_SUBDISTRICT) {
                    root_school_info.sub_district = text;
                } else if id_value.contains(SCHOOL_ID_DISTRICT) {
                    root_school_info.district = text;
                }
            }
        }
    }
    return Some(root_school_info);
}

fn get_address(
    parser: &HtmlParserService,
    element: &scraper::element_ref::ElementRef,
) -> Option<String> {
    let street_element_option = parser.select_from_element(element, SCHOOL_ADDRESS_STREET_ID);
    let zipcode_element_option = parser.select_from_element(element, SCHOOL_ADDRESS_PINCODE_ID);
    if street_element_option.is_none() || zipcode_element_option.is_none() {
        return None;
    }

    let street_element = street_element_option.unwrap()[0];
    let zipcode_element = zipcode_element_option.unwrap()[0];

    let address_street_option = parser.get_element_text(&street_element);
    let address_zipcode_option = parser.get_element_text(&zipcode_element);

    let address_street: String;
    let address_zipcode: String;

    match address_street_option {
        Some(street) => address_street = street,
        None => address_street = "".to_string(),
    };

    match address_zipcode_option {
        Some(zipcode) => address_zipcode = zipcode,
        None => address_zipcode = "".to_string(),
    };

    return Some(format!("{},{}", address_street, address_zipcode));
}

fn get_tel(
    parser: &HtmlParserService,
    element: &scraper::element_ref::ElementRef,
) -> Option<String> {
    let tel_element_option = parser.select_from_element(element, SCHOOL_TEL_ID);

    if tel_element_option.is_none() {
        return None;
    }

    let tel_element = tel_element_option.unwrap()[0];
    return parser.get_element_text(&tel_element);
}

fn get_fax(
    parser: &HtmlParserService,
    element: &scraper::element_ref::ElementRef,
) -> Option<String> {
    let fax_element_option = parser.select_from_element(element, SCHOOL_FAX_ID);

    if fax_element_option.is_none() {
        return None;
    }

    let fax_element = fax_element_option.unwrap()[0];
    return parser.get_element_text(&fax_element);
}

fn get_email(
    parser: &HtmlParserService,
    element: &scraper::element_ref::ElementRef,
) -> Option<String> {
    let email_element_option = parser.select_from_element(element, SCHOOL_EMAIL_ID);

    if email_element_option.is_none() {
        return None;
    }
    return parser.get_element_text(&email_element_option.unwrap()[0]);
}

fn get_weburl(
    parser: &HtmlParserService,
    element: &scraper::element_ref::ElementRef,
) -> Option<String> {
    let weburl_element_option = parser.select_from_element(element, SCHOOL_WEBURL_ID);

    if weburl_element_option.is_none() {
        return None;
    }
    return parser.get_element_text(&weburl_element_option.unwrap()[0]);
}

fn get_primary_contact(
    parser: &HtmlParserService,
    element: &scraper::element_ref::ElementRef,
) -> Option<String> {
    let primarycontact_element_option =
        parser.select_from_element(element, SCHOOL_PRIMARYCONTACT_ID);

    if primarycontact_element_option.is_none() {
        return None;
    }
    return parser.get_element_text(&primarycontact_element_option.unwrap()[0]);
}

fn get_languages(
    parser: &HtmlParserService,
    element: &scraper::element_ref::ElementRef,
) -> Option<Vec<String>> {
    let languages_element_option = parser.select_from_element(element, SCHOOL_LANGUAGES_ID);

    if languages_element_option.is_none() {
        return None;
    }
    let languages_option = parser.get_element_text(&languages_element_option.unwrap()[0]);
    if languages_option.is_none() {
        return None;
    }

    // languages are of the form: S1 Englisch/Französisch, S4 Englisch/Spanisch
    return Some(
        languages_option
            .unwrap()
            .split(",")
            .map(|x| -> String { x.to_string() })
            .collect(),
    );
}

#[derive(Default)]
struct RootSchoolInfo {
    name: String,
    school_type: String,
    sub_district: String,
    district: String,
}

//#endregion Private Area

// #[cfg(test)]
// mod tests {
//     // 1. Successful parsing for root html
//     #[test]
//     fn it_should_parse_root_html() {}
//     // 2. Unsuccessful parsing for root html
//     // 3. Successful selection of root selector
//     // 4. Unsuccessful selection of root selector
//     // 5. Parse <a> element:
//     //       No <a> element. Should return None
//     //       No href element, details_url in the tuple should be empty
//     //       href element, details_url in the tuple should have correct value
//     //       No text , school_id in the tuple should be empty
//     //       text, school_id in the tuple should have correct value
//     //
//     // 6. Parse <span> elements
//     //       No <span> elements, return None
//     //       Correct values for each of the 4 RootSchoolInfo (including empty and correct value)
//     // 7. get_school_root_results
//     //       should not panic
//     //       should return correct values
// }

/// Handles all region related functinality

use  crate::models::crawler_request::CrawlerRequest;
const VALID_REGIONS: [&str; 1] = ["berlin"];

pub fn is_valid_region(request: &CrawlerRequest) -> bool {
    // find takes a closure that returns true/false. Since it is a closure, region is wrapped in pipe (|)
    // operator

    // Why does the find predicate take &&region as an argument?
    // VALID_REGIONS.iter() returns an iterator. It is an iterator over &string
    // find takes a predicate which accepts a &Item as parameter. Sice Item already is a reference (&string)
    // the predicate has to take a &&
    let response = VALID_REGIONS
        .iter()
        .find(|&&region| region == request.region);

    // find returns Some(value) when the predicate is true for any element, else None.
    match response {
        Some(_x) => return true,
        None => return false,
    }
}

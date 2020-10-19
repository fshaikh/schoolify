use crate::models::crawler_request::CrawlerRequest;
use crate::services::config_service::get_crawler_request;

// App modules

/// Contains definitions for all code related to parsing command line arguments
// Standard modules
use std::env;

// #region - Public

pub fn read_args() -> Vec<String> {
    // Read the arguments. args() returns an iterator which is converted into a collection using "collect"
    return env::args().collect();
}

/// Parses Command line arguments and returns CrawlerRequest
///    Returns CLIParserError when invalid arguments are encountered
pub fn parse(args: Vec<String>) -> Result<CrawlerRequest, CLIParserError> {
    // Return error if arguments not as expected
    if args.len() < 2 {
        return make_error();
    }

    // Why am I using "clone" here?
    // 1. If we use region: args[1], this will try to move ownership to region.
    //    However, String does not implement a Copy trait, so fails to compile
    //
    // 2. We might then try a "Borrow" trait using references. So, region: &args[1].
    //     This will fail because of lifetime. This is a great feature of Rust which prevents "dangling references"
    //     issues. args and return value have different lifetimes. We are creating a reference in region pointing to
    //     args[1]. However, args has a shorter lifetime since its scope will be gone after the function returns.
    //
    let reg = args[1].clone();
    get_crawler_request(&reg).or_else(|err| make_error())
}

pub struct CLIParserError {
    pub message: String,
}

// #endregion

fn make_error() -> Result<CrawlerRequest, CLIParserError> {
    let error = CLIParserError {
        message: String::from("Invalid arguments supplied. Usage: cargo run berlin"),
    };
    return Err::<CrawlerRequest, CLIParserError>(error);
}

// #region - Tests
// #[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn it_should_return_region_correctly() {
        // let args: Vec<String> = vec![String::from("filename"), String::from("berlin")];
        // let parser_response = parse(args);
        // match parser_response {
        //     Ok(result) => assert_eq!(result.region, "berlin"),
        //     Err(err) => {}
        // }
    }

    // #[test]
    fn it_should_return_error_correctly() {
        let args: Vec<String> = vec![String::from("filename")];
        let parser_response = parse(args);
        match parser_response {
            Ok(_) => {}
            Err(err) => assert_eq!(
                err.message,
                "Invalid arguments supplied. Usage: cargo run berlin"
            ),
        }
    }
}
// #endregion - Tests

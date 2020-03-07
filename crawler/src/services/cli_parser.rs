use crate::utils::file_utils::get_text_file_data;
use crate::utils::json_parser::deserialize;
use crate::models::crawler_config::CrawlerConfig;
/// Contains definitions for all code related to parsing command line arguments
// Standard modules
use std::env;

// App modules
use crate::models::crawler_request::CrawlerRequest;

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
    let config_result = get_config(&reg);
    if config_result.is_none() {
        return make_error();
    }

    return Ok(CrawlerRequest {
        region: reg,
        config: config_result.unwrap(),
    });
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

fn get_config(region: &String) -> Option<CrawlerConfig> {
    let config_path = format!("./src/crawlers/{}/config.json", region);
    let result = get_text_file_data(config_path);

    return match result {
        Ok(data) => create_config(&data),
        Err(err) => {
            println!(
                "CLIParser::get_config - Error reading region crawler config file{}",
                err.message
            );
            None
        }
    };
}

fn create_config(data: &String) -> Option<CrawlerConfig> {
    let parsed_result = deserialize(&data);
    if parsed_result.is_err() {
        println!("Unable to parse config file: {}", data);
        return None;
    }
    return Some(CrawlerConfig::construct(parsed_result.unwrap()));
}

// #region - Tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_should_return_region_correctly() {
        let args: Vec<String> = vec![String::from("filename"), String::from("berlin")];
        let parser_response = crate::Services::CLIParser::parse(args);
        match parser_response {
            Ok(result) => assert_eq!(result.region, "berlin"),
            Err(err) => {}
        }
    }

    #[test]
    fn it_should_return_error_correctly() {
        let args: Vec<String> = vec![String::from("filename")];
        let parser_response = crate::services::CLIParser::parse(args);
        match parser_response {
            Ok(result) => {}
            Err(err) => assert_eq!(
                err.message,
                "Invalid arguments supplied. Usage: cargo run berlin"
            ),
        }
    }
}
// #endregion - Tests

/// Common error type
#[derive(Debug)]
pub struct Error {
    pub message: String,
}

pub fn make_error_message(errors: &Vec<Error>) -> String {
    for error in errors {
        println!("{}", error.message);
    }
    // TODO:
    return "".to_string();
}

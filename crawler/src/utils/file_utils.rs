/// File utilities
use std::fs::{read, read_to_string};
use std::path::Path;

pub struct FileError {
    pub path: String,
    pub message: String,
}

pub fn get_text_file_data(file_path: String) -> Result<String, FileError> {
    let path = Path::new(&file_path);

    let file_result = read_to_string(&path);
    match file_result {
        Ok(data) => Ok(data),
        Err(err) => {
            return Err::<String, FileError>(make_error(&file_path, err));
        }
    }
}

pub fn read_file(file_path: &String) -> Result<Vec<u8>, FileError> {
    read(&Path::new(file_path)).or_else(|err| Err(make_error(file_path, err)))
}

fn make_error(file_path: &String, err: std::io::Error) -> FileError {
    FileError {
        path: file_path.clone(),
        message: err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_load_text_file() {
        let config_path = format!("./src/crawlers/{}/config.json", "berlin");
        let result = get_text_file_data(config_path);
        let is_none = result.is_err();
        assert_eq!(is_none, false);
    }
}

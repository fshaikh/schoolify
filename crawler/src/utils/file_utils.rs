/// File utilities

pub struct FileError {
    pub path: String,
    pub message: String,
}

pub fn get_text_file_data(file_path: String) -> Result<String, FileError> {
    let path = std::path::Path::new(&file_path);

    let file_result = std::fs::read_to_string(&path);
    match file_result {
        Ok(data) => Ok(data),
        Err(err) => {
            return Err::<String, FileError>(make_error(&file_path, err));
        }
    }
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

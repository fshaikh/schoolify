/// HTTP(S) related functions
#[derive(Debug)]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub body: String,
}

pub struct HttpResponse {
    pub result: String,
    pub status_code: u32,
    pub error: String,
}

pub async fn get(request: &HttpRequest) -> HttpResponse {
    let get_response = get_core(&request).await;
    match get_response {
        Ok(res) => {
            return HttpResponse {
                result: res,
                status_code: 200,
                error: "".to_string(),
            }
        }
        Err(err) => {
            return HttpResponse {
                result: "".to_string(),
                status_code: 500,
                error: err.to_string(),
            }
        }
    }
}

async fn get_core(request: &HttpRequest) -> Result<String, reqwest::Error> {
    return Ok(reqwest::get(&request.url).await?.text().await?);
}

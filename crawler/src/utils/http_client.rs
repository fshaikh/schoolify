/// HTTP(S) related functions

use reqwest::{Url};

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

// pub struct JsonHttpResponse<T> {
//     pub result: T,
//     pub status_code: u32,
//     pub error: String,
// }

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

pub fn encode_url(url: &String) -> String {
    return Url::parse(url).unwrap().to_string();
}

// pub async fn get_json<T>(request: &HttpRequest) -> JsonHttpResponse<T> {
//     let get_response = get_json_core::<T>(&request).await;
//     match get_response {
//         Ok(res) => {
//             return JsonHttpResponse {
//                 result: res,
//                 status_code: 200,
//                 error: "".to_string(),
//             }
//         }
//         Err(err) => {
//             return JsonHttpResponse {
//                 result: err,
//                 status_code: 500,
//                 error: err.to_string(),
//             }
//         }
//     }
// }

async fn get_core(request: &HttpRequest) -> Result<String, reqwest::Error> {
    return Ok(reqwest::get(&request.url).await?.text().await?);
}

// async fn get_json_core<T: serde::de::Deserialize>(request: &HttpRequest) -> Result<T, reqwest::Error> {
//     return Ok(reqwest::get(&request.url).await?.json::<T>().await?)
// }

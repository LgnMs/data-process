use async_trait::async_trait;
use serde_json::Value;
use crate::data_processing::{Receive, Serde, Export};
use reqwest::{header::{self, HeaderName, HeaderValue}, Method};

#[derive(Default)]
pub struct Http {
    data: Value
}

struct HttpConfig {
    method: Method,
    headers: Option<Vec<(String, String)>>,
    body: Option<String>,
}

impl Http {
    fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Receive<HttpConfig, Http> for Http {
    async fn receive(&self, url: String, paramters: HttpConfig) -> Self {
        let mut headers = header::HeaderMap::new();
        let headers_vec = paramters.headers.unwrap_or(vec![]);

        for x in headers_vec {
            let name = HeaderName::from_bytes(x.0.as_bytes());
            let value = HeaderValue::from_bytes(x.1.as_bytes());
            if name.is_ok() && value.is_ok() {
                headers.insert(name.unwrap(), value.unwrap());
            } else {
                println!("{:?} 添加到header中失败", x);
            }
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let res = match paramters.method {
            Method::POST => {

                client.request(paramters.method, url)
                    .body(paramters.body.unwrap())
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
            },
            _ => {
                client.request(paramters.method, url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
            }
        };

        Self {
            data: res.unwrap().into()
        }
    }
}

impl Serde<Http> for Http {
    fn serde(&self) -> Self {
        // todo!();
        print!("{}", self.data);
        Self::default()
    }
}

impl Export<String> for Http {
    fn export(&self) -> String {
        println!("String");
        "Srr".into()
    }
}

#[actix_rt::test]
async fn test_http() {

    let http = Http::new()
        .receive(
            "http://10.20.198.88:8899/XXZX/api_e2b7f5af44f5b3e711?x-acs-apiCaller-uid=SMS8yzJuWOYPNxuK".into(),
            HttpConfig { 
                method: Method::POST,
                headers: Some(vec![
                    ("x-acs-apiCaller-uid".to_string(), "SMS8yzJuWOYPNxuK".to_string()),
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]), 
                body: Some(r#"
                    {
                        "start_time": "2023-03-01 00:00:00",
                        "end_time": "2023-03-15 23:59:59",
                        "pageOffset": "0",
                        "pageCounts": "500",
                        "token": "9652d443-238a-11ed-b8ed-005056bfedb1"
                    }
                "#.to_string())
            }
        )
        .await
        .serde();
   
    let _export1 = http.export();
}
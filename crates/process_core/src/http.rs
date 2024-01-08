use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::{
    header::{self, HeaderName, HeaderValue},
    Method,
};
use serde_json::{json, Value};

use crate::{process::{Export, Receive, Serde}, json::generate_new_map};

#[derive(Default, Debug, Clone)]
pub struct Http {
    data: Value,
    // 将数组0的数据映射给数组1的
    map_rules: Vec<[String; 2]>,
}

#[derive(Debug)]
pub struct HttpConfig {
    pub method: Method,
    pub headers: Option<Vec<(String, String)>>,
    pub body: Option<String>,
}

impl Http {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_map_rules(&mut self, map_rules: Vec<[String; 2]>) -> &mut Self {
        self.map_rules = map_rules;
        
        self
    }
}

#[async_trait]
impl Receive<HttpConfig, Result<Http>> for Http {
    async fn receive(&mut self, url: String, paramters: HttpConfig) -> Result<Http> {
        let mut headers = header::HeaderMap::new();
        let headers_vec = paramters.headers.as_ref().unwrap();

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
            .timeout(Duration::from_millis(5000))
            .build()?;

        tracing::debug!(
            "准备发起请求: client: {:?}\n url: {:?}\n paramters: {:?}",
            client,
            url,
            paramters
        );

        let res = match paramters.method {
            Method::POST => {
                client
                    .request(paramters.method, url)
                    .body(paramters.body.unwrap_or("".to_string()))
                    .send()
                    .await?
                    .json()
                    .await?
            }
            _ => {
                client
                    .request(paramters.method, url)
                    .send()
                    .await?
                    .json()
                    .await?
            }
        };

        self.data = res;

        Ok(self.clone())
    }
}


impl Serde for Http {
    type Target = Result<Http>;
    fn serde(&mut self) -> Result<Http> {
        let map_rules = &self.map_rules;
        let origin_data = &self.data;
        let mut new_data = json!({});
        generate_new_map(&map_rules, &mut new_data, origin_data)?;

        self.data = new_data;
        Ok(self.clone())
    }
}

impl Export<String> for Http {
    fn export(&mut self) -> String {
        println!("String");
        "Srr".into()
    }
}

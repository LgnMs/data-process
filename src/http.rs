use std::{time::Duration, clone};

use async_trait::async_trait;
use serde_json::{Value, Map, value::Index, json};
use anyhow::Result;
use reqwest::{header::{self, HeaderName, HeaderValue}, Method};

use crate::data_processing::{Receive, Serde, Export};

#[derive(Default, Debug, Clone)]
pub struct Http {
    data: Value,

    // // 将数组0的数据映射给数组1的
    // map_vec: Vec<[String; 2]>,
}

#[derive(Debug)]
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
impl Receive<HttpConfig, Result<Http>> for Http {
    async fn receive(&mut self, url: String, paramters: HttpConfig) -> Result<Self> {
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

        tracing::debug!("准备发起请求: client: {:?}\n url: {:?}\n paramters: {:?}", client, url, paramters);

        let res = match paramters.method {
            Method::POST => {
                client.request(paramters.method, url)
                    .body(paramters.body.unwrap_or("".to_string()))
                    .send()
                    .await?
                    .text()
                    .await?
            },
            _ => {
                client.request(paramters.method, url)
                    .send()
                    .await?
                    .text()
                    .await?
            }
        };

        self.data = res.into();

        Ok(self.clone())
    }
}

fn find_value(key: &String, data: &Value) -> Value {
    let mut target_value = &data.clone();

    let mut last_has_index = 0;
    for i in 0..key.len() {
        if key.get(i..i+1).unwrap() == "." {
            target_value =  target_value.get(&key[last_has_index..i]).unwrap();
            last_has_index = i;
        }
    }

    if last_has_index + 1 == key.len() {
        target_value = target_value.get(key).unwrap();
    } else {
        println!("last: {}, {}", &key[last_has_index+1..key.len()], target_value);
        target_value = target_value.get(&key[last_has_index+1..key.len()]).unwrap();
    }
    
    target_value.clone()
}
/// 根据映射规则生成新的Map数据
fn generate_new_map<'a>(map_rules: &'a Vec<[String; 2]>, new_data: &'a mut Value, old_data: &'a Value) {
    
    let mut temp_data = new_data;

    for map_rule in map_rules {
        let origin = map_rule.get(0).unwrap();
        let target = map_rule.get(1).unwrap();

        let mut last_has_index = 0;
        for i in 0..target.len() {
            if target.get(i..i+1).unwrap() == "." {
                temp_data.as_object_mut().unwrap().insert(target[last_has_index..i].to_string(), json!({}));
                temp_data = temp_data.as_object_mut().unwrap().get_mut(&target[last_has_index..i].to_string()).unwrap();
                last_has_index = i;
            }
        }

        if last_has_index + 1 == target.len() {
            // 这是a -> b形式
            temp_data.as_object_mut().unwrap().insert(target.clone(), find_value(origin, old_data));
        } else {
            // 这是a.b -> b.c形式
            temp_data.as_object_mut().unwrap().insert(target.get(last_has_index+1..target.len()).unwrap().to_owned(), find_value(origin, old_data));
        }
    }

}

impl Serde for Http {
    type Target = Http;
    fn serde(&mut self) -> Self {
        Self::default()
    }
}

impl Export<String> for Http {
    fn export(&mut self) -> String {
        println!("String");
        "Srr".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_http() {
        // tracing_subscriber::fmt()
        //     // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        //     // will be written to stdout.
        //     .with_max_level(Level::TRACE)
        //     // sets this to be the default, global subscriber for this application.
        //     .init();
    
        let mut http = Http::new()
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
            .unwrap()
            .serde();
       
        let _export1 = http.export();
    }

    #[test]
    fn serde_test() {
        let map_rules = vec![
            ["a".to_string(), "b".to_string()],
            ["b.c".to_string(), "c.d".to_string()],
        ];
        let mut new_data = json!({});
        let old_data = json!({
            "a": 1,
            "b": {
                "c": 2
            }
        });

        generate_new_map(&map_rules, &mut new_data, &old_data);

        println!("{}", new_data);
    }

}
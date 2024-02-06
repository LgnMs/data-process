use std::time::Duration;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::{
    header::{self, HeaderName, HeaderValue},
    Method,
};
use serde_json::{json, Value};
use tracing::{debug, error};

use crate::{
    json::{find_value, generate_new_map},
    process::{Export, Receive, Serde},
};

#[derive(Default, Debug, Clone)]
pub struct Http {
    pub data: Value,
    /// 将数组0的数据映射给数组1的
    pub map_rules: Vec<[String; 2]>,
    /// 导出字符模板
    /// ```js
    /// 例如：data = { data: [{"id: 1, "name": "name1"}, {"id: 2, "name": "name2"}] }
    /// "INSERT INTO table_name (column1, column2) VALUES (${data#id}, ${data#name})" ->
    /// ["INSERT INTO table_name (column1, column2) VALUES (1, name1)", "INSERT INTO table_name (column1, column2) VALUES (2, name2)"]
    /// ````
    pub template_string: Option<String>,
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
    pub fn set_data(&mut self, data: Value) -> &mut Self {
        self.data = data;

        self
    }

    pub fn add_map_rules(&mut self, map_rules: Vec<[String; 2]>) -> &mut Self {
        self.map_rules = map_rules;

        self
    }

    pub fn set_template_string(&mut self, template_string: String) -> &mut Self {
        self.template_string = Some(template_string.trim().to_string());

        self
    }
}

#[async_trait]
impl Receive<HttpConfig, Result<Http>> for Http {
    async fn receive(&mut self, url: String, parameters: HttpConfig) -> Result<Http> {
        let mut headers = header::HeaderMap::new();
        let headers_vec = parameters.headers.as_ref().unwrap();

        for x in headers_vec {
            let name = HeaderName::from_bytes(x.0.as_bytes());
            let value = HeaderValue::from_bytes(x.1.as_bytes());
            if name.is_ok() && value.is_ok() {
                headers.insert(name.unwrap(), value.unwrap());
            } else {
                error!("{:?} 添加到header中失败", x);
            }
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_millis(15000))
            .build()?;

        debug!(
            "准备发起请求: client: {:?}\n url: {:?}\n parameters: {:?}",
            client, url, parameters
        );

        let res = match parameters.method {
            Method::POST => {
                client
                    .request(parameters.method, url)
                    .body(parameters.body.unwrap_or("".to_string()))
                    .send()
                    .await?
                    .text()
                    .await?
            }
            _ => {
                client
                    .request(parameters.method, url)
                    .send()
                    .await?
                    .text()
                    .await?
            }
        };

        debug!("返回数据: {:?}\n ", res);

        match serde_json::from_slice(res.as_bytes()) {
            Ok(x) => self.data = x,
            Err(err) => {
                let err_str = format!("返回的数据无法被序列化 请检查api是否能被正常调用 {}", err);
                error!("{}", err_str);
                return Err(anyhow!(err_str));
            }
        };

        Ok(self.clone())
    }
}

impl Serde for Http {
    type Target = Result<Http>;

    fn serde(&mut self) -> Self::Target {
        let map_rules = &self.map_rules;
        let origin_data = &self.data;
        let mut new_data = json!({});
        generate_new_map(&map_rules, &mut new_data, origin_data)?;

        self.data = new_data;
        Ok(self.clone())
    }
}

type SQLString = String;

impl Export for Http {
    type Target = Result<Vec<SQLString>>;

    fn export(&mut self) -> Self::Target {
        let template_sql = self
            .template_string
            .as_ref()
            .ok_or(anyhow!("未设置template_string"))?;

        let mut temp_index_vec = vec![];

        for i in 0..template_sql.len() {
            let s = &template_sql[i..i + 1];
            if s == "{" && i != 0 && &template_sql[i - 1..i] == "$" {
                temp_index_vec.push(i);
            } else if s == "}" {
                if let Some(last) = temp_index_vec.last() {
                    let last_i = last.clone();
                    if &template_sql[last_i..last_i + 1] == "{" {
                        temp_index_vec.push(i);
                    }
                }
            }
        }

        let mut key_vec = vec![];
        let mut i = 0;
        while i < temp_index_vec.len() {
            let one_index = temp_index_vec[i] - 1; // 取"{"前$的索引，所以减1
            let two_index = temp_index_vec[i + 1];

            key_vec.push(template_sql[one_index..two_index + 1].to_string());

            i += 2;
        }

        let mut result_vec: Vec<String> = vec![];

        for key in key_vec {
            let rel_key = &key[2..key.len() - 1];
            let value = find_value(rel_key, &self.data)?;
            if let Some(index) = rel_key.chars().position(|c| c == '#') {
                let data_list = value.as_array().unwrap();

                let mut j = 0;
                for old_item in data_list {
                    let item = find_value(&rel_key[index + 1..], old_item)?;

                    if let Some(template) = result_vec.get(j) {
                        result_vec[j] = template.replace(&key, item.as_str().unwrap_or("null"));
                    } else {
                        result_vec.push(template_sql.replace(&key, item.as_str().unwrap_or("null")))
                    }
                    j += 1;
                }
            } else {
                if let Some(template) = result_vec.get_mut(0) {
                    result_vec[0] = template.replace(&key, value.as_str().unwrap());
                } else {
                    result_vec.push(template_sql.replace(&key, value.as_str().unwrap()))
                }
            }
        }

        Ok(result_vec)
    }
}

use std::time::Duration;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use reqwest::{
    header::{self, HeaderName, HeaderValue},
    Method,
};
use serde_json::{json, Value};
use tracing::{error, debug};

use crate::{process::{Export, Receive, Serde}, json::{generate_new_map, find_value}};

#[derive(Default, Debug, Clone)]
pub struct Http {
    pub data: Value,
    /// 将数组0的数据映射给数组1的
    pub map_rules: Vec<[String; 2]>,
    /// 导出字符模板
    /// ```
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
    async fn receive(&mut self, url: String, paramters: HttpConfig) -> Result<Http> {
        let mut headers = header::HeaderMap::new();
        let headers_vec = paramters.headers.as_ref().unwrap();

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
            .timeout(Duration::from_millis(5000))
            .build()?;

        debug!(
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
    type Target = Result<SQLString>;
    
    fn export(&mut self) -> Self::Target {
        let template_sql = self.template_string.as_ref().ok_or(anyhow!("未设置template_string"))?;

        let mut temp_index_vec = vec![];

        for i in 0..template_sql.len() {
            let s = &template_sql[i..i+1];
            if s == "{" && i != 0 && &template_sql[i-1..i] == "$" {
                temp_index_vec.push(i);
            } else if s == "}" {
                if let Some(last) = temp_index_vec.last() {
                    let last_i = last.clone();
                    if &template_sql[last_i..last_i+1] == "{" {
                        temp_index_vec.push(i);
                    }
                }
            }
        }

        let mut key_vec = vec![];
        let mut i = 0;
        while i < temp_index_vec.len() {
            let one_index = temp_index_vec[i] - 1; // 取"{"前$的索引，所以减1
            let two_index = temp_index_vec[i+1];

            key_vec.push(template_sql[one_index..two_index+1].to_string());

            i += 2;
        }

        let mut result_vec: Vec<String> = vec![];

        let get_sql_string = |
            list: &mut Vec<String>,
            data_list: Option<&Vec<Value>>
        | {
            if list.len() == 0 {
                return template_sql.clone();
            } else {
                let first = list.first().unwrap().clone();
                if data_list.is_some() && data_list.unwrap().len() >= list.len() {
                    list.clear();
                }
                return first;
            }
        };

        for key in key_vec {
            let rel_key = &key[2..key.len() - 1];
            let value = find_value(&rel_key.to_string(), &self.data)?;
            if let Some(index) = rel_key.chars().position(|c| c == '#' ) {
                let data_list =  value.as_array().unwrap();
                let template = get_sql_string(&mut result_vec, Some(data_list));

                for old_item in data_list {
                    let item = find_value(&rel_key[index+1..].to_string(), old_item)?;
                    let sql_str = template.replace(&key, item.as_str().unwrap());
                    result_vec.push(sql_str);
                }
            } else {
                let mut sql_str = get_sql_string(&mut result_vec, None);
                sql_str = sql_str.replace(&key, value.as_str().unwrap());
                result_vec.push(sql_str);
            }
        }

        Ok(json!(result_vec).to_string())
    }
}

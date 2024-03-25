/// 从http请求中获取数据并处理
use std::time::Duration;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::{
    header::{self, HeaderName, HeaderValue},
    Method,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, error};

use crate::json::flat_nested_object;
use crate::{
    json::{find_value, map_data},
    process::{Export, Receive, Serde},
};

#[derive(Default, Debug, Clone)]
pub struct Http {
    pub data: Value,
    /// 将数组0的数据映射给数组1的
    pub map_rules: Option<Vec<[String; 2]>>,
    pub nested_config: Option<Vec<NestedConfig>>,
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NestedConfig {
    root_key: String,
    children_key: String,
    id_key: String,
}

impl Http {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_data(&mut self, data: Value) -> &mut Self {
        self.data = data;

        self
    }

    pub fn set_map_rules(&mut self, map_rules: Vec<[String; 2]>) -> &mut Self {
        self.map_rules = Some(map_rules);

        self
    }

    pub fn set_nested_config(&mut self, nested_config: Vec<NestedConfig>) -> &mut Self {
        self.nested_config = Some(nested_config);

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
                    .timeout(Duration::from_secs(120))
                    .send()
                    .await?
                    .text()
                    .await?
            }
            _ => {
                client
                    .request(parameters.method, url)
                    .timeout(Duration::from_secs(120))
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
                let err_str = format!(
                    "返回的数据 {res} 无法被序列化 请检查api是否能被正常调用 {}",
                    err
                );
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
        // 处理接收到的数据，用于展开父子结构的嵌套数据
        if let Some(config_list) = &self.nested_config {
            for item in config_list {
                match flat_nested_object(
                    &self.data,
                    item.root_key.as_str(),
                    item.children_key.as_str(),
                    item.id_key.as_str(),
                ) {
                    Err(_) => {}
                    Ok(x) => {
                        let a = x.to_string();
                        println!("a {a}");
                        self.data = x;
                    }
                }
            }
        }

        if let Some(map_rules) = &self.map_rules {
            self.data = map_data(&self.data, map_rules)?;
        }

        Ok(self.clone())
    }
}

pub(crate) type SQLString = String;

#[async_trait]
impl Export for Http {
    type Target = Result<Vec<SQLString>>;

    async fn export(&mut self) -> Self::Target {
        let template_sql = self
            .template_string
            .as_ref()
            .ok_or(anyhow!("未设置template_string"))?;

        generate_sql_list(template_sql, &self.data)
    }
}

pub fn generate_sql_list(template_sql: &String, data: &Value) -> Result<Vec<String>> {
    let mut temp_index_vec: Vec<(usize, char)> = vec![];

    let mut pre_char = '0';
    for (i, s) in template_sql.char_indices() {
        if s == '{' && pre_char == '$' {
            temp_index_vec.push((i, s));
        } else if s == '}' {
            if let Some((_, c)) = temp_index_vec.last() {
                if *c == '{' {
                    temp_index_vec.push((i, s));
                }
            }
        }
        pre_char = s;
    }

    let mut key_vec = vec![];
    let mut i = 0;
    while i < temp_index_vec.len() {
        let one_index = temp_index_vec[i].0 - "$".as_bytes().len(); // 取"{"前$的索引，所以减1
        let two_index = temp_index_vec[i + 1].0;

        key_vec.push(template_sql[one_index..two_index + "}".as_bytes().len()].to_string());

        i += 2;
    }

    let mut result_vec: Vec<String> = vec![];

    for key in key_vec {
        let rel_key = &key[2..key.len() - 1];
        let value = find_value(rel_key, data, true)
            .map_err(|err| anyhow!("{err} 未在rel_key: {rel_key} data:{}中找到数据", data))?;
        if let Some(list) = value.as_array() {
            for i in 0..list.len() {
                let item: &str;
                let temp_string = list[i].to_string();
                if let Some(x) = list[i].as_str() {
                    item = x;
                } else {
                    item = temp_string.as_str();
                }

                if result_vec.get(i).is_none() {
                    result_vec.push(template_sql.replace(key.as_str(), item));
                } else {
                    result_vec[i] = result_vec[i].replace(key.as_str(), item);
                }
            }
        }
    }

    Ok(result_vec)
}

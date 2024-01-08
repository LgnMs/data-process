use std::time::Duration;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::{
    header::{self, HeaderName, HeaderValue},
    Method,
};
use serde_json::{json, Value, Map};

use crate::data_processing::{Export, Receive, Serde};

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
                    .text()
                    .await?
            }
            _ => {
                client
                    .request(paramters.method, url)
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

fn find_value(key: &String, data: &Value) -> Result<Value> {
    let mut target_value = &data.clone();
    let mut last_has_index = 0;
    let mut has_dot = false;
    let mut has_sharp = false;
    let err = || -> String {
        format!("未在数据 {data} 中找到键值：{key}")
    };

    for i in 0..key.len() {
        // 这是a.b -> b.c形式
        let str = key.get(i..i + 1).unwrap();
        let current_key = key[last_has_index..i].to_string();

        if str == "." {
            has_dot = true;
            target_value = target_value
                .get(current_key)
                .ok_or(anyhow!(err()))?;
            last_has_index = i + 1;
        } else if str == "#" {
            has_sharp = true;
            target_value = target_value
                .get(current_key)
                .ok_or(anyhow!(err()))?;
            last_has_index = i + 1;
            // 数组形式只返回数组本身，后续值获取交给回调函数处理
            // 例如data#a.b只会返回data的值
            break;
        }
    }

    if has_dot {
        // 这是a.b -> b.c形式
        target_value = target_value
            .get(&key[last_has_index..key.len()])
            .ok_or(anyhow!(err()))?;
    } else if has_sharp {
        // 这是a#b -> b#c形式 什么都不做
        // target_value = target_value;
    } else {
        // 这是a -> b形式
        target_value = target_value
            .get(key)
            .ok_or(anyhow!(err()))?;
    }
    Ok(target_value.clone())
}
/// 根据映射规则生成新的Map数据
fn generate_new_map<'a>(
    map_rules: &'a Vec<[String; 2]>,
    new_data: &'a mut Value,
    old_data: &'a Value,
) -> Result<()> {
    let err = format!(
        "数据格式与转换规则不匹配 data: {old_data} map_rules: {:?}",
        map_rules
    );

    for map_rule in map_rules {
        let mut temp_data = &mut *new_data;
        let origin = map_rule.get(0).unwrap();
        let target = map_rule.get(1).unwrap();

        let mut last_has_index = 0;
        let mut has_dot = false;
        let mut has_sharp = false;

        for i in 0..target.len() {
            // 这是a.b -> b.c形式
            let str = target.get(i..i + 1).unwrap();
            let key = target[last_has_index..i].to_string();
            if str == "." {
                has_dot = true;
                temp_data
                    .as_object_mut()
                    .ok_or(anyhow!(err.clone()))?
                    .insert(key.clone(), json!({}));
                temp_data = temp_data.as_object_mut().unwrap().get_mut(&key).unwrap();
                last_has_index = i + 1;
            }
            if str == "#" {
                has_sharp = true;
                
                let temp_data =  temp_data
                    .as_object_mut()
                    .ok_or(anyhow!(err.clone()))?;
                let current_item = temp_data.get_mut(&key);

                if current_item.is_none() {
                    let init_insert = || -> Result<Value> {
                        let new_origin_data = find_value(origin, old_data)?;

                        if let Some(x) = new_origin_data.as_array() {
                            let last_key = origin[origin.as_str().find("#").unwrap()+1..].to_string();
                            let mut array = vec![];
                            for item in x {
                                let current_value = find_value(&last_key, item)?;
                                let target_last_key = target[i+1..].to_string();
                                if target_last_key.contains('.') || target_last_key.contains('#') {
                                    let mut val = json!({});
                                    let map_rules = vec![
                                        [last_key.clone(), target_last_key.clone()],
                                    ];
                                    generate_new_map(&map_rules, &mut val, &item)?;
                                    array.push(val);
                                } else {
                                    let mut map = Map::new();
                                    map.insert(target[i+1..].to_string(), current_value);
                                    array.push(json!(map));
                                }
                            }
                            Ok(json!(array))
                        } else {
                            let mut map = Map::new();
                            map.insert(target[i+1..].to_string(), new_origin_data);
                            Ok(json!([map]))
                        }
                    };
                    let value = init_insert()?;
                    temp_data.insert(key.clone(), value);
                } else if let Some(item) = current_item {
                    let modify = |e: &mut Value| -> Result<()> {
                        let current_array = e.as_array_mut().unwrap();
                        let new_origin_data = find_value(origin, old_data)?;
                        
                        // 当获取到的原始数据是array形式，就循环根据规则进行映射
                        if let Some(x) = new_origin_data.as_array() {
                            let last_key = origin[origin.as_str().find("#").unwrap()+1..].to_string();
                            for j in 0..x.len() {
                                let item = x.get(j).unwrap();
                                // 因为current_array初始化时的数量是由原始数据中的获取到的数组数量决定的，所以他们的索引值一定一一对应
                                let current_array_item = current_array.get_mut(j).unwrap();
                                let current_value = find_value(&last_key, item)?;

                                // current_array_item.as_object_mut().unwrap().insert(target[i+1..].to_string(), current_value);
                                let target_last_key = target[i+1..].to_string();
                                if target_last_key.contains('.') || target_last_key.contains('#') {
                                    let mut val = json!({});
                                    let map_rules = vec![
                                        [last_key.clone(), target_last_key.clone()],
                                    ];
                                    generate_new_map(&map_rules, &mut val, &item).unwrap();
                                    current_array_item.as_object_mut().unwrap().append(val.as_object_mut().unwrap());
                                } else {
                                    current_array_item.as_object_mut().unwrap().insert(target_last_key.clone(), current_value);
                                }
                                
                            }
                        } 
                        else // 当获取到的原始数据不是形式，直接写入新创建的数组中
                        {
                            for item in current_array {
                                item.as_object_mut().unwrap().insert(target[i+1..].to_string(), new_origin_data.clone());
                            }
                        }
                        Ok(())
                    };

                    modify(item)?;
                }

                last_has_index = i + 1;
                break;
            }
        }

        if has_dot {
            // 这是a.b -> b.c形式
            temp_data
                .as_object_mut()
                .unwrap()
                .insert(
                    target.get(last_has_index..target.len()).unwrap().to_owned(),
                    find_value(origin, old_data)?,
                );
        } else if has_sharp {
            // 这是a#b -> b#c形式 什么都不做
            // 已经在上面的判断中处理完毕
        } else {
            // 这是a -> b形式
            temp_data
                .as_object_mut()
                .ok_or(anyhow!(err.clone()))?
                .insert(target.clone(), find_value(origin, old_data)?);
        }
    }

    Ok(())
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
    fn serde_json_test() {
        let old_data = json!({
            "a": 1,
            "b": {
                "c": 2,
                "d": {
                    "e": 3
                }
            }
        });

        let map_rules = vec![["a".to_string(), "b".to_string()]];
        let mut new_data = json!({});
        let _ = generate_new_map(&map_rules, &mut new_data, &old_data);
        assert_eq!(new_data, json!({"b": 1}));

        let map_rules2 = vec![["b.c".to_string(), "e.d".to_string()]];
        let mut new_data2 = json!({});
        let _ = generate_new_map(&map_rules2, &mut new_data2, &old_data);
        assert_eq!(
            new_data2,
            json!({
                "e": {"d": 2}
            })
        );

        let map_rules3 = vec![["b.d.e".to_string(), "c.d.e.f".to_string()]];
        let mut new_data3 = json!({});
        let _ = generate_new_map(&map_rules3, &mut new_data3, &old_data);
        assert_eq!(
            new_data3,
            json!({
                "c": {
                    "d": {
                        "e": {
                            "f": 3
                        }
                    }
                }
            })
        );

        let map_rules4 = vec![
            ["a".to_string(), "b".to_string()],
            ["b.c".to_string(), "e.d".to_string()],
        ];
        let mut new_data4 = json!({});
        let _ = generate_new_map(&map_rules4, &mut new_data4, &old_data);
        assert_eq!(
            new_data4,
            json!({
                "b": 1,
                "e": {"d": 2}
            })
        );

        let map_rules5 = vec![
            ["a".to_string(), "b".to_string()],
            ["b.c".to_string(), "e.d".to_string()],
            ["b.d.e".to_string(), "c.d.e.f".to_string()],
        ];
        let mut new_data5 = json!({});
        let _ = generate_new_map(&map_rules5, &mut new_data5, &old_data);
        assert_eq!(
            new_data5,
            json!({
                "b": 1,
                "e": {"d": 2},
                "c": {
                    "d": {
                        "e": {
                            "f": 3
                        }
                    }
                }
            })
        );
    }

    #[test]
    fn serde_json_err_test() {
        let old_data = json!({
            "a": 1,
            "b": {
                "c": 2,
                "d": {
                    "e": 3
                }
            }
        });

        let map_rules = vec![["c".to_string(), "b".to_string()]];
        let mut new_data = json!({});
        match generate_new_map(&map_rules, &mut new_data, &old_data) {
            Ok(_) => assert_eq!(new_data, json!({"b": 1})),
            Err(err) => println!("{err}"),
        }

        let map_rules = vec![["a.c".to_string(), "b".to_string()]];
        let mut new_data2 = json!({});
        match generate_new_map(&map_rules, &mut new_data2, &old_data) {
            Ok(_) => assert_eq!(new_data2, json!({"b": 1})),
            Err(err) => println!("{err}"),
        }
    }

    #[test]
    fn serde_json_array_test() {
        let old_data = json!({
            "test": 1,
            "data": [
                {
                    "a": 1,
                    "b": 2
                },
                {
                    "a": 2,
                    "b": 3
                }
            ]
        });
        let map_rules = vec![
            ["data#a".to_string(), "res#aa".to_string()],
            ["data#b".to_string(), "res#bb".to_string()],
        ];
        let mut new_data = json!({});
        let _ = generate_new_map(&map_rules, &mut new_data, &old_data);
        assert_eq!(
            new_data,
            json!({
                "res": [
                    {
                        "aa": 1,
                        "bb": 2
                    },
                    {
                        "aa": 2,
                        "bb": 3
                    }
                ]
            })
        );

        let map_rules2 = vec![
            ["test".to_string(), "res#aa".to_string()],
            ["test".to_string(), "res#bb".to_string()],
        ];
        let mut new_data2 = json!({});
        let _ = generate_new_map(&map_rules2, &mut new_data2, &old_data);
        assert_eq!(
            new_data2,
            json!({
                "res": [
                    {
                        "aa": 1,
                        "bb": 1
                    }
                ]
            })
        );


        let old_data2 = json!({
            "test": 1,
            "data": [
                {
                    "a": {
                        "b": 1
                    },
                    "b": {
                        "c": 2
                    }
                },
                {
                    "a": {
                        "b": 2
                    },
                    "b": {
                        "c": 3
                    }
                }
            ]
        });
        let map_rules3 = vec![
            ["data#a.b".to_string(), "res#aa".to_string()],
            ["data#b.c".to_string(), "res#bb".to_string()],
        ];
        let mut new_data3 = json!({});
        let _ = generate_new_map(&map_rules3, &mut new_data3, &old_data2);
        assert_eq!(
            new_data3,
            json!({
                "res": [
                    {
                        "aa": 1,
                        "bb": 2
                    },
                    {
                        "aa": 2,
                        "bb": 3
                    }
                ]
            })
        );

        let old_data3 = json!({
            "test": 1,
            "data": [
                {
                    "a": {
                        "b": 1
                    },
                    "b": {
                        "c": 2
                    }
                },
                {
                    "a": {
                        "b": 2
                    },
                    "b": {
                        "c": 3
                    }
                }
            ]
        });

        let map_rules4 = vec![
            ["data#a.b".to_string(), "res#aa.bb".to_string()],
            ["data#b.c".to_string(), "res#bb.cc".to_string()],
        ];
        let mut new_data4 = json!({});
        let _ = generate_new_map(&map_rules4, &mut new_data4, &old_data3);
        assert_eq!(
            new_data4,
            json!({
                "res": [
                    {
                        "aa": {
                            "bb": 1
                        },
                        "bb": {
                            "cc": 2
                        }
                    },
                    {
                        "aa": {
                            "bb": 2
                        },
                        "bb": {
                            "cc": 3
                        }
                    }
                ]
            })
        );
    }
}

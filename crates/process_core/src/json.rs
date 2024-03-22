use anyhow::anyhow;
use serde_json::{json, Value};
use tracing::error;

/// should_flat 是否展开获取到的数据
pub fn find_value(key: &str, value: &Value, should_flat: bool) -> anyhow::Result<Value> {
    let mut current_key = key;
    let current_index: &str;
    let mut current_value = Some(value.clone());

    if let Some(index) = current_key.find(".") {
        current_index = &current_key[..index];
        current_value = Some(
            current_value
                .as_ref()
                .ok_or(anyhow!("查找数据失败"))?
                .get(current_index)
                .ok_or(anyhow!("查找数据失败"))?
                .clone(),
        );
        current_key = &current_key[index + 1..];
        return find_value(
            current_key,
            &current_value.ok_or(anyhow!("查找数据失败"))?,
            should_flat,
        );
    } else if let Some(index) = current_key.find("#") {
        current_index = &current_key[..index];

        current_value = match current_value
            .as_ref()
            .ok_or(anyhow!("查找数据失败"))?
            .get(current_index)
        {
            None => match current_value.ok_or(anyhow!("查找数据失败"))?.as_array() {
                None => {
                    error!("未找到索引 {current_index} 对应的数据");
                    None
                }
                Some(list) => {
                    let value = list
                        .iter()
                        .map(|x| find_value(current_index, x, should_flat).unwrap_or(json!(null)))
                        .collect::<Vec<Value>>();
                    Some(json!(value))
                }
            },
            Some(val) => Some(val.clone()),
        };
        current_key = &current_key[index + 1..];

        return match current_value {
            None => Err(anyhow!("")),
            Some(x) => find_value(current_key, &x, should_flat),
        };
    } else {
        current_index = current_key;

        current_value = Some(re_find(
            current_index,
            &current_value.ok_or(anyhow!("查找数据失败"))?,
            should_flat,
        )?);
    }

    current_value.ok_or(anyhow!("查找数据失败"))
}

pub fn re_find(key: &str, value: &Value, should_flat: bool) -> anyhow::Result<Value> {
    match value.get(key) {
        None => match value.as_array() {
            None => Err(anyhow!("re_find {value}数据失败\n")),
            Some(list) => {
                let mut has_array = false;
                let mut list = list
                    .iter()
                    .map(|x| match re_find(key, x, should_flat) {
                        Err(_) => {
                            has_array = false;
                            json!(null)
                        }
                        Ok(x) => {
                            if x.is_array() {
                                has_array = true;
                            }
                            x
                        }
                    })
                    .collect::<Vec<Value>>();
                if has_array && should_flat {
                    list = list
                        .into_iter()
                        .flat_map(|x| x.as_array().unwrap().clone())
                        .collect();
                }

                Ok(json!(list))
            }
        },
        Some(val) => Ok(val.clone()),
    }
}

/// 只支持同一层级结构数据转换
pub fn map_data(origin_data: &Value, map_rules: &Vec<[String; 2]>) -> anyhow::Result<Value> {
    let mut new_value = json!({});

    for rule in map_rules {
        let origin = rule[0].as_str();
        let target = rule[1].as_str();

        get_target_rule_data(origin, target, origin_data, &mut new_value);
    }

    if new_value == json!({}) {
        Err(anyhow!("map_data 数据转换失败"))
    } else {
        Ok(new_value)
    }
}

fn get_target_rule_data(o_key: &str, t_key: &str, origin_data: &Value, value: &mut Value) {
    let mut key = t_key;
    let current_key: &str;

    if let Some(index) = key.find('.') {
        if let Some(v_map) = value.as_object_mut() {
            current_key = &key[..index];
            key = &key[index + 1..];
            if !v_map.contains_key(current_key) {
                v_map.insert(current_key.to_string(), json!({}));
            }
            let current_val = v_map.get_mut(current_key).unwrap();
            get_target_rule_data(o_key, key, origin_data, current_val);
        } else if let Some(v_array) = value.as_array_mut() {
            if v_array.is_empty() {
                let mut new_val = json!({});
                get_target_rule_data(o_key, key, origin_data, &mut new_val);
                v_array.push(new_val);
            } else {
                for item in v_array {
                    get_target_rule_data(o_key, key, origin_data, item);
                }
            }
        }
    } else if let Some(index) = key.find('#') {
        if let Some(v_map) = value.as_object_mut() {
            current_key = &key[..index];
            key = &key[index + 1..];

            if !v_map.contains_key(current_key) {
                v_map.insert(current_key.to_string(), json!([]));
            }
            let current_val = v_map.get_mut(current_key).unwrap();
            get_target_rule_data(o_key, key, origin_data, current_val);
        } else if let Some(v_array) = value.as_array_mut() {
            if v_array.is_empty() {
                let mut new_val = json!({});
                get_target_rule_data(o_key, key, origin_data, &mut new_val);
                v_array.push(new_val);
            } else {
                for item in v_array {
                    get_target_rule_data(o_key, key, origin_data, item);
                }
            }
        }
    } else {
        current_key = t_key;
        let target_value = find_value(o_key, origin_data, true).unwrap_or(json!(null));
        if let Some(res_list) = target_value.as_array() {
            let mut i = 0;
            for item in res_list {
                if let Some(v_map) = value.as_object_mut() {
                    v_map.insert(current_key.to_string(), item.clone());
                } else if let Some(v_array) = value.as_array_mut() {
                    if v_array.is_empty() {
                        v_array.push(json!(
                            {
                                current_key: item.clone(),
                            }
                        ));
                    } else {
                        if v_array.get(i).is_none() {
                            v_array.push(v_array[0].clone())
                        }

                        let current_item = &mut v_array[i];

                        current_item
                            .as_object_mut()
                            .unwrap()
                            .insert(current_key.to_string(), item.clone());
                    }
                }
                i += 1;
            }
        }
    }
}

pub fn flat_nested_object(
    value: &Value,
    root_key: &str,
    children_key: &str,
    id_key: &str,
) -> anyhow::Result<Value> {
    let root_value = find_value(root_key, value, true);

    let mut data_list = vec![];
    if let Ok(mut root_value) = root_value {
        if let Some(list) = root_value.as_array_mut() {
            for item in list {
                flat_nested_callback(item, None, &mut data_list, children_key, id_key);
            }
        }

        let mut new_value = value.clone();
        let mut current_key = root_key;
        let mut current_item = &mut new_value;
        while let Some(index) = current_key.find(".") {
            current_item = current_item.get_mut(&current_key[..index]).unwrap();
            current_key = &current_key[index + 1..];
        }

        current_item
            .as_object_mut()
            .unwrap()
            .insert(current_key.to_string(), json!(data_list));

        return Ok(new_value);
    }
    Err(anyhow!("展开嵌套数据失败： {value}"))
}

fn flat_nested_callback(
    value: &mut Value,
    parent: Option<&Value>,
    data_list: &mut Vec<Value>,
    children_key: &str,
    id_key: &str,
) {
    let mut new_value = value.clone();
    new_value.as_object_mut().unwrap().remove(children_key);
    new_value
        .as_object_mut()
        .unwrap()
        .entry(format!("parent_{id_key}"))
        .or_insert_with(|| match parent {
            None => {
                json!(null)
            }
            Some(x) => x[id_key].clone(),
        });
    data_list.push(new_value);

    let children = find_value(children_key, value, true);
    if let Ok(mut child) = children {
        if let Some(list) = child.as_array_mut() {
            for item in list {
                flat_nested_callback(item, Some(&value), data_list, children_key, id_key);
            }
        }
    }
}

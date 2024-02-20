use serde_json::{json, Value};
use tracing::error;

//
// pub fn find_value<T: Borrow<str>>(key_o: T, data: &Value) -> Result<Value> {
//     let key: &str = key_o.borrow();
//     let mut target_value = &data.clone();
//     let mut last_has_index = 0;
//     let mut has_dot = false;
//     let mut has_sharp = false;
//     let err = || -> Error {
//         let err_str = format!("未在数据 {data} 中找到键值：{key}");
//         error!("{}", err_str);
//         anyhow!(err_str)
//     };
//
//     for i in 0..key.len() {
//         // 这是a.b -> b.c形式
//         let str = key.get(i..i + 1).unwrap();
//         let current_key = key[last_has_index..i].to_string();
//
//         if str == "." {
//             has_dot = true;
//             target_value = target_value.get(current_key).ok_or_else(err)?;
//             last_has_index = i + 1;
//         } else if str == "#" {
//             has_sharp = true;
//             target_value = target_value.get(current_key).ok_or_else(err)?;
//             last_has_index = i + 1;
//             // 数组形式只返回数组本身，后续值获取交给回调函数处理
//             // 例如data#a.b只会返回data的值
//             break;
//         }
//     }
//
//     if has_sharp {
//         // 这是a#b -> b#c形式 什么都不做
//         // target_value = target_value;
//     } else if has_dot {
//         // 这是a.b -> b.c形式
//         target_value = target_value
//             .get(&key[last_has_index..key.len()])
//             .ok_or_else(err)?;
//     } else {
//         // 这是a -> b形式
//         target_value = target_value.get(key).ok_or_else(err)?;
//     }
//     Ok(target_value.clone())
// }
//
// /// 根据映射规则生成新的Map数据
// pub fn generate_new_map<'a>(
//     map_rules: &'a Vec<[String; 2]>,
//     new_data: &'a mut Value,
//     old_data: &'a Value,
// ) -> Result<()> {
//     let err = || -> Error {
//         let err_str = format!(
//             "数据格式与转换规则不匹配 data: {old_data} map_rules: {:?}",
//             map_rules
//         );
//         error!("{}", err_str);
//         anyhow!(err_str)
//     };
//
//     for map_rule in map_rules {
//         let mut temp_data = &mut *new_data;
//         let origin = map_rule.get(0).unwrap();
//         let target = map_rule.get(1).unwrap();
//
//         let mut last_has_index = 0;
//         let mut has_dot = false;
//         let mut has_sharp = false;
//
//         for i in 0..target.len() {
//             // 这是a.b -> b.c形式
//             let str = target.get(i..i + 1).unwrap();
//             let key = target[last_has_index..i].to_string();
//             if str == "." {
//                 has_dot = true;
//                 if temp_data.as_object_mut().unwrap().get(&key).is_none() {
//                     temp_data
//                         .as_object_mut()
//                         .ok_or_else(err)?
//                         .insert(key.clone(), json!({}));
//                 }
//                 temp_data = temp_data.as_object_mut().unwrap().get_mut(&key).unwrap();
//                 last_has_index = i + 1;
//             }
//             if str == "#" {
//                 has_sharp = true;
//                 let temp_data = temp_data.as_object_mut().ok_or_else(err)?;
//
//                 let current_item = temp_data.get_mut(&key);
//
//                 if current_item.is_none() {
//                     let init_insert = || -> Result<Value> {
//                         let new_origin_data = find_value(origin.borrow(), old_data)?;
//                         if let Some(x) = new_origin_data.as_array() {
//                             let last_key =
//                                 origin[origin.as_str().find("#").unwrap() + 1..].to_string();
//                             let mut array = vec![];
//                             for item in x {
//                                 let current_value = find_value(last_key.borrow(), item)?;
//                                 let target_last_key = target[i + 1..].to_string();
//                                 if target_last_key.contains('.') || target_last_key.contains('#') {
//                                     let mut val = json!({});
//                                     let map_rules =
//                                         vec![[last_key.clone(), target_last_key.clone()]];
//
//                                     generate_new_map(&map_rules, &mut val, &item)?;
//                                     array.push(val);
//                                 } else {
//                                     let mut map = Map::new();
//                                     map.insert(target[i + 1..].to_string(), current_value);
//                                     array.push(json!(map));
//                                 }
//                             }
//                             Ok(json!(array))
//                         } else {
//                             let mut map = Map::new();
//                             map.insert(target[i + 1..].to_string(), new_origin_data);
//                             Ok(json!([map]))
//                         }
//                     };
//                     let value = init_insert()?;
//                     temp_data.insert(key.clone(), value);
//                 } else if let Some(item) = current_item {
//                     let modify = |e: &mut Value| -> Result<()> {
//                         let current_array = e.as_array_mut().unwrap();
//                         let new_origin_data = find_value(origin.borrow(), old_data)?;
//
//                         // 当获取到的原始数据是array形式，就循环根据规则进行映射
//                         if let Some(x) = new_origin_data.as_array() {
//                             let last_key =
//                                 origin[origin.as_str().find("#").unwrap() + 1..].to_string();
//                             for j in 0..x.len() {
//                                 let item = x.get(j).unwrap();
//                                 // 因为current_array初始化时的数量是由原始数据中的获取到的数组数量决定的，所以他们的索引值一定一一对应
//                                 let current_array_item = current_array.get_mut(j).unwrap();
//                                 let current_value = find_value(last_key.borrow(), item)?;
//
//                                 // current_array_item.as_object_mut().unwrap().insert(target[i+1..].to_string(), current_value);
//                                 let target_last_key = target[i + 1..].to_string();
//                                 if target_last_key.contains('.') {
//                                     let mut val = json!({});
//                                     let map_rules =
//                                         vec![[last_key.clone(), target_last_key.clone()]];
//                                     generate_new_map(&map_rules, &mut val, &item).unwrap();
//                                     current_array_item
//                                         .as_object_mut()
//                                         .unwrap()
//                                         .append(val.as_object_mut().unwrap());
//                                 } else if target_last_key.contains('#') {
//                                     let mut val = current_array_item.clone();
//                                     let map_rules =
//                                         vec![[last_key.clone(), target_last_key.clone()]];
//                                     generate_new_map(&map_rules, &mut val, &item).unwrap();
//                                     current_array_item
//                                         .as_object_mut()
//                                         .unwrap()
//                                         .append(val.as_object_mut().unwrap());
//                                 } else {
//                                     current_array_item
//                                         .as_object_mut()
//                                         .unwrap()
//                                         .insert(target_last_key.clone(), current_value);
//                                 }
//                             }
//                         } else
//                         // 当获取到的原始数据不是形式，直接写入新创建的数组中
//                         {
//                             for item in current_array {
//                                 item.as_object_mut()
//                                     .unwrap()
//                                     .insert(target[i + 1..].to_string(), new_origin_data.clone());
//                             }
//                         }
//                         Ok(())
//                     };
//
//                     modify(item)?;
//                 }
//
//                 last_has_index = i + 1;
//                 break;
//             }
//         }
//
//         if has_sharp {
//             // 这是a#b -> b#c形式 什么都不做
//             // 已经在上面的判断中处理完毕
//         } else if has_dot {
//             // 这是a.b -> b.c形式
//             temp_data.as_object_mut().unwrap().insert(
//                 target.get(last_has_index..target.len()).unwrap().to_owned(),
//                 find_value(origin.borrow(), old_data)?,
//             );
//         } else {
//             // 这是a -> b形式
//             temp_data
//                 .as_object_mut()
//                 .ok_or_else(err)?
//                 .insert(target.clone(), find_value(origin.borrow(), old_data)?);
//         }
//     }
//
//     Ok(())
// }

/// should_flat 是否展开最后一层
pub fn find_value(key: &str, value: &Value, should_flat: bool) -> Option<Value> {
    let mut current_key = key;
    let current_index: &str;
    let mut current_value = Some(value.clone());

    if let Some(index) = current_key.find(".") {
        current_index = &current_key[..index];
        current_value = Some(
            current_value
                .as_ref()
                .unwrap()
                .get(current_index)
                .unwrap()
                .clone(),
        );
        current_key = &current_key[index + 1..];
        return find_value(current_key, &current_value.unwrap(), should_flat);
    } else if let Some(index) = current_key.find("#") {
        current_index = &current_key[..index];

        current_value = match current_value.as_ref().unwrap().get(current_index) {
            None => match current_value.unwrap().as_array() {
                None => {
                    error!("未找到索引 {current_index} 对应的数据");
                    None
                },
                Some(list) => {
                    let value = list
                        .iter()
                        .map(|x| find_value(current_index, x, should_flat).unwrap())
                        .collect::<Vec<Value>>();
                    Some(json!(value))
                }
            },
            Some(val) => Some(val.clone()),
        };
        current_key = &current_key[index + 1..];

        return match current_value {
            None => None,
            Some(x) => {
                find_value(current_key, &x, should_flat)
            }
        }
    } else {
        current_index = current_key;

        current_value = re_find(current_index, &current_value.unwrap(), should_flat);
    }

    match current_value {
        None => None,
        Some(x) => Some(x.clone()),
    }
}

pub fn re_find(key: &str, value: &Value, should_flat: bool) -> Option<Value> {
    match value.get(key) {
        None => match value.as_array() {
            None => None,
            Some(list) => {
                let mut has_array = false;
                let mut list = list
                    .iter()
                    .map(|x| {
                        match re_find(key, x, should_flat) {
                            None => {
                                has_array = false;
                                json!(null)
                            },
                            Some(x) => {
                                if x.is_array() {
                                    has_array = true;
                                }
                                x
                            }
                        }
                    })
                    .collect::<Vec<Value>>();
                if has_array && should_flat {
                    list = list
                        .into_iter()
                        .flat_map(|x| x.as_array().unwrap().clone())
                        .collect();
                }

                Some(json!(list))
            }
        },
        Some(val) => Some(val.clone()),
    }
}

/// 只支持同一层级结构数据转换
pub fn map_data(origin_data: &Value, map_rules: &Vec<[String; 2]>) -> Option<Value> {
    let mut new_value = json!({});

    for rule in map_rules {
        let origin = rule[0].as_str();
        let target = rule[1].as_str();

        get_target_rule_data(origin, target, origin_data, &mut new_value);
    }

    if new_value == json!({}) {
        None
    } else {
        Some(new_value)
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

                        current_item.as_object_mut()
                            .unwrap()
                            .insert(current_key.to_string(), item.clone());
                    }
                }
                i += 1;
            }

        }
    }
}


pub fn flat_nested_object(value: &Value, root_key: &str, children_key: &str, id_key: &str) -> Option<Value> {
    let root_value = find_value(root_key, value, true);

    let mut data_list = vec![];
    if let Some(mut root_value) = root_value {
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
            current_key = &current_key[index+1..];
        }

        current_item
            .as_object_mut()
            .unwrap()
            .insert(current_key.to_string(), json!(data_list));

        return Some(new_value);
    }
    None
}

fn flat_nested_callback(value: &mut Value, parent: Option<&Value>, data_list: &mut Vec<Value>, children_key: &str, id_key: &str) {
    let mut new_value = value.clone();
    new_value
        .as_object_mut()
        .unwrap()
        .remove(children_key);
    new_value
        .as_object_mut()
        .unwrap()
        .entry(format!("parent_{id_key}"))
        .or_insert_with(|| {
            match parent {
                None => {
                    json!(null)
                }
                Some(x) => {
                    x[id_key].clone()
                }
            }
        });
    data_list.push(new_value);

    let children = find_value(children_key, value, true);
    if let Some(mut child) = children {
        if let Some(list) = child.as_array_mut() {
            for item in list {
                flat_nested_callback(item, Some(&value), data_list, children_key, id_key);
            }
        }
    }
}
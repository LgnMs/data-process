use process_core::json::find_value;
use serde_json::{json, Value};

#[test]
fn find_value_test() {
    let origin_data = json!({
        "data":[
            {
                "id": 1,
                "list": [
                    {
                        "a": 2,
                        "b": 2
                    }
                ],
                "children": [
                    {
                        "id": 2,
                        "list": [
                            {
                                "a": 2,
                                "b": 2
                            }
                        ],
                    }
                ]
            },
            {
                "id": 2,
                "list": [
                    {
                        "a": 3,
                        "b": 3
                    }
                ],
                "children": [
                    {
                        "id": 3,
                        "list": [
                            {
                                "a": 3,
                                "b": 3
                            }
                        ],
                    }
                ]
            }
        ]
    });

    let a = find_value("data#id", &origin_data);
    let b = find_value("data#list#a", &origin_data);
    let c = find_value("data#children#list#a", &origin_data);

    assert_eq!(a, Some(json!(vec![1, 2])));
    assert_eq!(b, Some(json!(vec![2, 3])));
    assert_eq!(c, Some(json!(vec![2, 3])));

    let origin_data2 = json!({
        "result": {
            "code": 200,
            "data":[
                {
                    "id": 1,
                    "list": [
                        {
                            "a": 2,
                            "b": 2
                        }
                    ],
                    "children": [
                        {
                            "id": 2,
                            "list": [
                                {
                                    "a": 2,
                                    "b": 2
                                }
                            ],
                        }
                    ]
                }
            ]
        }
    });

    let d = find_value("result.data#id", &origin_data2);
    assert_eq!(d, Some(json!(vec![1])));

    let e = find_value("result.code", &origin_data2);
    assert_eq!(e, Some(json!(200)));
}

#[test]
fn map_data_test() {
    let origin_data = json!({
        "data":[
            {
                "id": 1,
                "list": [
                    {
                        "a": "a1",
                        "b": "b1"
                    }
                ],
                "children": [
                    {
                        "id": 2,
                        "list": [
                            {
                                "a": "a2",
                                "b": "b2"
                            }
                        ],
                    }
                ]
            }
        ]
    });

    let transform_rules = vec![
        ["data#id", "data#parent_id"],
        ["data#children#id", "data#id"],
        ["data#children#list#a", "data#a"],
        ["data#children#list#b", "data#b"],
    ];

    let new_data = map_data(origin_data, transform_rules);

    let b = json!({
        "data":[
            {
                "parent_id": 1,
                "id": 2,
                "a": "a2",
                "b": "b2"
            }
        ]
    });
    assert_eq!(new_data, Some(b));
}

fn map_data(origin_data: Value, map_rules: Vec<[&str; 2]>) -> Option<Value> {
    let mut new_value = json!({});

    for rule in map_rules {
        let origin = rule[0];
        let target = rule[1];

        get_target_rule_data(origin, target, &origin_data, &mut new_value);
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
        let target_value = find_value(o_key, origin_data).unwrap_or(json!(null));

        if let Some(res_list) = target_value.as_array() {
            for item in res_list {
                if let Some(v_map) = value.as_object_mut() {
                    v_map.insert(current_key.to_string(), item.clone());
                } else if let Some(v_array) = value.as_array_mut() {
                    if v_array.is_empty() {
                        v_array.push(json!({
                                current_key: item.clone(),
                            }
                        ));
                    } else {
                        for item2 in v_array {
                            item2
                                .as_object_mut()
                                .unwrap()
                                .insert(current_key.to_string(), item.clone());
                        }
                    }
                }
            }
        }
    }
}

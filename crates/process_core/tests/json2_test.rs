use serde_json::{json, Value};
use process_core::json::new_find_value;

#[test]
fn new_find_value_test() {

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

    let a = new_find_value("data#id", origin_data.clone());
    let b = new_find_value("data#list#a", origin_data.clone());
    let c = new_find_value("data#children#list#a", origin_data);

    assert_eq!(a, Some(json!(vec![1, 2])));
    assert_eq!(b, Some(json!(vec![2, 3])));
    assert_eq!(c, Some(json!(vec![2, 3])));

}

fn map_data_test() {
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
            }
        ]
    });

    let transform_rules = vec![
        ["data#list#children#id", "data#list#id"],
        ["data#list#children#list#a", "data#list#a"],
        ["data#list#children#list#b", "data#list#b"],
    ];

    let new_data = map_data(origin_data, transform_rules);

    let b = json!({
        "data":[
            {
                "list": [
                    {
                        "id": 1,
                        "a": 2,
                        "b": 2
                    }
                ]
            },

        ]
    });
    assert_eq!(new_data, Some(b));
}

fn map_data(origin_data: Value, map_rules: Vec<[&str;2]>) -> Option<Value> {
    // TODO


    None
}
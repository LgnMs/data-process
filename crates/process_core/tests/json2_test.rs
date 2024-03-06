use process_core::json::{find_value, flat_nested_object, map_data};
use serde_json::json;

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
                    },
                    {
                        "id": 4,
                        "list": [
                            {
                                "a": 4,
                                "b": 4
                            }
                        ],
                    },
                ]
            }
        ]
    });

    let a = find_value("data#id", &origin_data, true).expect("err");
    let b = find_value("data#list#a", &origin_data, true).expect("err");
    let c = find_value("data#children#list#a", &origin_data, true).expect("err");

    assert_eq!(a, json!(vec![1, 2]));
    assert_eq!(b, json!(vec![2, 3]));
    assert_eq!(c, json!(vec![2, 3, 4]));

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

    let d = find_value("result.data#id", &origin_data2, true).expect("err");
    assert_eq!(d, json!(vec![1]));

    let e = find_value("result.code", &origin_data2, true).expect("err");
    assert_eq!(e, json!(200));

    let origin_data3 = json!([
        {"a": 1}, {"a": 2}, {"a": 3}
    ]);

    let e = find_value("a", &origin_data3, true).expect("err");
    assert_eq!(e, json!(vec![1, 2, 3]));
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
            },
            {
                "id": 2,
                "list": [
                    {
                        "a": "a2",
                        "b": "b2"
                    }
                ],
                "children": [
                    {
                        "id": 3,
                        "list": [
                            {
                                "a": "a3",
                                "b": "b3"
                            }
                        ],
                    }
                ]
            }
        ]
    });

    let transform_rules = vec![
        ["data#children#list#a".to_string(), "data#a".to_string()],
        ["data#children#list#b".to_string(), "data#b".to_string()],
    ];

    let new_data = map_data(&origin_data, &transform_rules).expect("err");

    let b = json!({
        "data":[
            {
                "a": "a2",
                "b": "b2"
            },
            {
                "a": "a3",
                "b": "b3"
            }
        ]
    });
    assert_eq!(new_data, b);
}

#[test]
fn test_flat_nested_object() {
    let origin_data = json!({
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
                        },
                    ]
                }
            ]
        }
    });

    let new_data = flat_nested_object(&origin_data, "result.data", "children", "id");

    assert_eq!(
        new_data.unwrap(),
        json!({
            "result": {
                "code": 200,
                "data":[
                    {
                        "parent_id": null,
                        "id": 1,
                        "list": [
                            {
                                "a": 2,
                                "b": 2
                            }
                        ],
                    },
                    {
                        "parent_id": 1,
                        "id": 2,
                        "list": [
                            {
                                "a": 2,
                                "b": 2
                            }
                        ],
                    },
                    {
                        "parent_id": 2,
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
        })
    )
}

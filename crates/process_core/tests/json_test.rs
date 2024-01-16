use process_core::json::generate_new_map;
use serde_json::json;

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

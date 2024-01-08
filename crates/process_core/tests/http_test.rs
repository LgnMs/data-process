
use process_core::{http::*, process::*};
use reqwest::Method;

#[actix_rt::test]
    async fn test_http() {
        // tracing_subscriber::fmt()
        //     // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        //     // will be written to stdout.
        //     .with_max_level(Level::TRACE)
        //     // sets this to be the default, global subscriber for this application.
        //     .init();

        let mut http = Http::new()
            .add_map_rules(vec![
                ["code".to_string(), "code2".to_string()],
                ["data.result#pkid".to_string(), "res.data#id".to_string()],
                ["data.result#citycode".to_string(), "res.data#citycode".to_string()],
            ])
            .receive(
                "http://10.20.198.88:8899/XXZX/api_4d9bd6a24a7b97961".into(),
                HttpConfig {
                    method: Method::POST,
                    headers: Some(vec![
                        ("x-acs-apiCaller-uid".to_string(), "SMS8yzJuWOYPNxuK".to_string()),
                        ("Content-Type".to_string(), "application/json".to_string()),
                    ]),
                    body: Some(r#"
                        {
                            "start_time": "2024-01-01 00:00:00",
                            "end_time": "2024-01-05 23:00:00",
                            "citycode": "513300",
                             "pagesize": 50,
                            "token": "9652d515-238a-11ed-b8ed-005056bfedb1"
                        }
                    "#.to_string())
                }
            )
            .await
            .unwrap()
            .serde()
            .unwrap();

        let _export1 = http.export();
    }

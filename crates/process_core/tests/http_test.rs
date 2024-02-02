use anyhow::Result;
use process_core::{http::*, process::*};
use serde_json::json;

#[actix_rt::test]
async fn test_http() -> Result<()> {
    let mut http = Http::new()
        // .receive(
        //     "http://127.0.0.1:8000/mock/test_data_2".into(),
        //     HttpConfig {
        //         method: Method::POST,
        //         headers: Some(vec![
        //             (
        //                 "x-acs-apiCaller-uid".to_string(),
        //                 "SMS8yzJuWOYPNxuK".to_string(),
        //             ),
        //             ("Content-Type".to_string(), "application/json".to_string()),
        //         ]),
        //         body: Some(r#"{"current":1,"page_size":1}"#.to_string()),
        //     },
        // )
        // .await?
        .set_data(json!({"code":"SUCCESS", "data": {"result": [{"pkid": 1, "citycode": "2", "avg_no2_degree": "3" }]}}))
        .add_map_rules(vec![
            ["code".to_string(), "code2".to_string()],
            ["data.result#pkid".to_string(), "res.data#id".to_string()],
            [
                "data.result#citycode".to_string(),
                "res.data#citycode".to_string(),
            ],
            [
                "data.result#avg_no2_degree".to_string(),
                "res.data#no2".to_string(),
            ],
        ])
        .serde()?;
    println!("Http: {:?}", http);
    let export1 = http
        .set_template_string(
            "INSERT INTO table_name (column1, column2) VALUES (${res.data#id}, ${res.data#no2})"
                .to_string(),
        )
        .export()?;
    println!("export: {:?}", export1);
    Ok(())
}

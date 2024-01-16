use anyhow::Result;
use process_core::{http::*, process::*};
use reqwest::Method;
use tracing::Level;

#[actix_rt::test]
async fn test_http() -> Result<()> {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        .with_line_number(true)
        .with_file(true)
        // sets this to be the default, global subscriber for this application.
        .init();
    let mut http = Http::new()
        .receive(
            "http://10.20.198.88:8899/XXZX/api_4d9bd6a24a7b97961".into(),
            HttpConfig {
                method: Method::POST,
                headers: Some(vec![
                    (
                        "x-acs-apiCaller-uid".to_string(),
                        "SMS8yzJuWOYPNxuK".to_string(),
                    ),
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]),
                body: Some(
                    r#"
                    {
                        "start_time": "2024-01-01 00:00:00",
                        "end_time": "2024-01-05 23:00:00",
                        "citycode": "513300",
                            "pagesize": 50,
                        "token": "9652d515-238a-11ed-b8ed-005056bfedb1"
                    }
                "#
                    .to_string(),
                ),
            },
        )
        .await?
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
    println!("export: {export1}");
    Ok(())
}

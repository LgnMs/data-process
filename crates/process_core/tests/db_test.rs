use process_core::db;
use process_core::db::{DataSource, Db, DbConfig};
use process_core::process::{Export, Receive, Serde};
use sea_orm::Database;

#[actix_rt::test]
async fn db_test() -> anyhow::Result<()> {
    let mut db = Db::new();
    let cache_db_url = "postgres://postgres:123456@127.0.0.1:5432/data_process_cache".to_string();
    let cache_conn = Database::connect(cache_db_url)
        .await
        .expect("Cache Database connection failed");

    let target = DataSource {
        host: "127.0.0.1".to_string(),
        port: "5432".to_string(),
        user: "postgres".to_string(),
        password: "123456".to_string(),
        database_name: "data_process_cache".to_string(),
        database_type: db::Database::POSTGRES,
    };
    let res = db
        .receive(
            r#"SELECT id, parent_code, parent_ci_id, code, "name", unit, value, ci_id, type_name
FROM public.test_data_32;"#
                .to_string(),
            DbConfig { conn: cache_conn },
        )
        .await?
        .set_template_string(
            r#"INSERT INTO public.sync_test_table (code, naem) VALUES('${code}', '${name}');"#
                .to_string(),
        )
        .set_target_db_source_config(target)
        .export()
        .await;

    match res {
        Ok(x) => {
            println!("{:?}", x);
        }
        Err(err) => {
            print!("{}", err);
        }
    }

    Ok(())
}

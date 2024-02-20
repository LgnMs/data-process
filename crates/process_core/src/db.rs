use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sea_orm::{ConnectionTrait, DatabaseConnection, FromQueryResult, JsonValue, Statement};
use serde_json::{json, Value};

use crate::http::generate_sql_list;
use crate::process::Export;
use crate::process::Receive;
use crate::process::Serde;

/// 从数据库中获取数据并处理

#[derive(Debug, Clone)]
pub struct Db {
    pub data: Option<Value>,
    // pub map_rules: Option<Vec<[String; 2]>>,
    pub template_string: Option<String>,
    pub target_db_source_config: Option<DataSource>,
}

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub conn: DatabaseConnection,
}

/// 管理多数据源，然后执行SQL
#[derive(Debug, Clone)]
pub struct DataSource {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub database_name: String,
    pub database_type: Database,
}

#[derive(Debug, Clone)]
pub enum Database {
    MYSQL,
    MSSQL,
    KINGBASE,
    POSTGRES,
    ORACLE,
}

impl Db {
    pub fn new() -> Self {
        Self {
            data: None,
            template_string: None,
            target_db_source_config: None,
        }
    }

    pub fn set_template_string(&mut self, template_string: String) -> &mut Self {
        self.template_string = Some(template_string.trim().to_string());

        self
    }

    pub fn set_target_db_source_config(
        &mut self,
        target_db_source_config: DataSource,
    ) -> &mut Self {
        self.target_db_source_config = Some(target_db_source_config);

        self
    }
}

async fn execute_sql(db_source: &DataSource, query_sql_list: Vec<String>) -> Result<()> {
    match db_source.database_type {
        Database::POSTGRES => {
            let db_url = format!(
                "postgres://{}:{}@{}/{}",
                db_source.user, db_source.password, db_source.host, db_source.database_name
            );
            let db = sea_orm::Database::connect(db_url.as_str()).await?;

            for sql in query_sql_list {
                println!("sql {sql}");
                db.execute(Statement::from_string(db.get_database_backend(), sql))
                    .await?;
            }
        }
        Database::MYSQL => {}
        Database::KINGBASE => {}
        Database::ORACLE => {}
        Database::MSSQL => {}
    };

    Ok(())
}
#[async_trait]
impl Receive<DbConfig, Result<Db>> for Db {
    async fn receive(&mut self, query_sql: String, parameters: DbConfig) -> Result<Db> {
        let data: Vec<JsonValue> = JsonValue::find_by_statement(Statement::from_sql_and_values(
            parameters.conn.get_database_backend(),
            query_sql,
            [],
        ))
        .all(&parameters.conn)
        .await?;

        self.data = Some(json!(data));

        Ok(self.clone())
    }
}

impl Serde for Db {
    type Target = Result<Db>;

    fn serde(&mut self) -> Self::Target {
        // if let Some(map_rules) = &self.map_rules {
        //     if let Some(data) = &self.data {
        //         let x = map_data(data, map_rules).ok_or(anyhow!("映射数据不成功"))?;
        //         self.data = Some(x);
        //     }
        // }

        Ok(self.clone())
    }
}

#[async_trait]
impl Export for Db {
    type Target = Result<()>;

    async fn export(&mut self) -> Self::Target {
        if self.data.is_none() {
            return Err(anyhow!("self.data 中没有数据"));
        }
        let data = self.data.as_ref().unwrap();

        println!("Value {data}");
        let template_sql = self
            .template_string
            .as_ref()
            .ok_or(anyhow!("未设置template_string"))?;

        let sql_list = generate_sql_list(template_sql, data)?;

        if let Some(db_source) = &self.target_db_source_config {
            // tokio::task::spawn(async move {
            //     if let Err(err) = execute_sql(&db, sql_list).await {
            //         error!("{}", anyhow!(err));
            //     }
            // });
            println!("db_source {db_source:?}");
            execute_sql(&db_source, sql_list).await?;
        }
        Ok(())
    }
}

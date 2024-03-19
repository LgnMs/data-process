use anyhow::{anyhow, Result};
use async_trait::async_trait;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use process_jdbc::common::{ExecuteJDBC, JDBC};
use process_jdbc::kingbase::Kingbase;
use process_jdbc::mssql::MSSQL;
use process_jdbc::oracle::Oracle;
use sea_orm::{ConnectionTrait, FromQueryResult, JsonValue, Statement};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{debug, warn};

use crate::http::generate_sql_list;
use crate::process::Export;
use crate::process::Receive;
use crate::process::Serde;

/// 从数据库中获取数据并处理

#[derive(Debug, Clone)]
pub struct Db {
    pub data: Option<Value>,
    pub db_source_config: Option<DataSource>,
    // pub map_rules: Option<Vec<[String; 2]>>,
    pub template_string: Option<String>,
    pub target_db_source_config: Option<DataSource>,
}

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub db_source_config: DataSource,
}

/// 管理多数据源，然后执行SQL
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataSource {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub database_name: String,
    pub table_schema: Option<String>,
    pub database_type: Database,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
            db_source_config: None,
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

pub async fn execute_sql(db_source: &DataSource, query_sql_list: Vec<String>) -> Result<()> {
    debug!("db_source {:?}", db_source);
    let password = decode_db_password(&db_source.password);
    match db_source.database_type {
        Database::POSTGRES => {
            let db_url = format!(
                "postgres://{}:{}@{}:{}/{}",
                db_source.user,
                password,
                db_source.host,
                db_source.port,
                db_source.database_name
            );
            let db = sea_orm::Database::connect(db_url.as_str()).await?;

            for sql in query_sql_list {
                db.execute(Statement::from_string(db.get_database_backend(), sql))
                    .await?;
            }
            Ok(())
        }
        Database::MYSQL => {
            let db_url = format!(
                "mysql://{}:{}@{}:{}/{}",
                db_source.user,
                password,
                db_source.host,
                db_source.port,
                db_source.database_name
            );
            let db = sea_orm::Database::connect(db_url.as_str()).await?;

            for sql in query_sql_list {
                db.execute(Statement::from_string(db.get_database_backend(), sql))
                    .await?;
            }
            Ok(())
        }
        Database::MSSQL => {
            let db_url = format!(
                "jdbc:sqlserver://{}:{};databaseName={}",
                db_source.host, db_source.port, db_source.database_name,
            );
            let mut conn = MSSQL::new()?;

            conn.connect(
                &db_url,
                db_source.user.as_str(),
                password.as_str(),
            )
            .map_err(|err| anyhow!("数据库连接失败！: {err}"))?;

            for sql in query_sql_list {
                conn.execute_update(&sql)
                    .map_err(|err| anyhow!("数据库查询失败！: {err}"))?;
            }
            Ok(())
        }
        Database::ORACLE => {
            let db_url = format!(
                "jdbc:oracle:thin:@//{}:{}/{}",
                db_source.host, db_source.port, db_source.database_name,
            );
            let mut conn = Oracle::new()?;

            conn.connect(
                &db_url,
                db_source.user.as_str(),
                password.as_str(),
            )
            .map_err(|err| anyhow!("数据库连接失败！: {err}"))?;

            for sql in query_sql_list {
                conn.execute_update(&sql)
                    .map_err(|err| anyhow!("数据库查询失败！: {err}"))?;
            }
            Ok(())
        }
        Database::KINGBASE => {
            let db_url = format!(
                "jdbc:kingbase8://{}:{}/{}",
                db_source.host, db_source.port, db_source.database_name,
            );
            let mut conn = Kingbase::new()?;

            conn.connect(
                &db_url,
                db_source.user.as_str(),
                password.as_str(),
            )
            .map_err(|err| anyhow!("数据库连接失败！: {err}"))?;

            for sql in query_sql_list {
                conn.execute_update(&sql)
                    .map_err(|err| anyhow!("数据库查询失败！: {err}"))?;
            }
            Ok(())
        }
    }
}


pub async fn find_all_sql(db_source: &DataSource, query_sql: String) -> Result<Vec<Value>> {
    debug!("db_source {:?}", db_source);
    if let Some(index) = query_sql.to_lowercase().find("select") {
        if index != 0 {
            return Err(anyhow!("这条语句不是查询语句！"));
        }
    } else {
        return Err(anyhow!("这条语句不是查询语句！"));
    }
    let password = decode_db_password(&db_source.password);
    match db_source.database_type {
        Database::POSTGRES => {
            let db_url = format!(
                "postgres://{}:{}@{}:{}/{}",
                db_source.user,
                password,
                db_source.host,
                db_source.port,
                db_source.database_name
            );
            let db = sea_orm::Database::connect(db_url.as_str()).await?;

            let data: Vec<JsonValue> = JsonValue::find_by_statement(
                Statement::from_sql_and_values(db.get_database_backend(), query_sql, []),
            )
            .all(&db)
            .await?;

            Ok(data)
        }
        Database::MYSQL => {
            let db_url = format!(
                "mysql://{}:{}@{}:{}/{}",
                db_source.user,
                password,
                db_source.host,
                db_source.port,
                db_source.database_name
            );
            let db = sea_orm::Database::connect(db_url.as_str()).await?;

            let data: Vec<JsonValue> = JsonValue::find_by_statement(
                Statement::from_sql_and_values(db.get_database_backend(), query_sql, []),
            )
            .all(&db)
            .await?;

            Ok(data)
        }
        Database::KINGBASE => {
            let db_url = format!(
                "jdbc:kingbase8://{}:{}/{}",
                db_source.host, db_source.port, db_source.database_name,
            );
            let mut conn = Kingbase::new()?;

            conn.connect(
                &db_url,
                db_source.user.as_str(),
                password.as_str(),
            )
            .map_err(|err| anyhow!("数据库连接失败！: {err}"))?;

            let data = conn
                .execute_query(&query_sql)
                .map_err(|err| anyhow!("数据库查询失败！: {err}"))?;
            Ok(data)
        }
        Database::MSSQL => {
            let db_url = format!(
                "jdbc:sqlserver://{}:{};DatabaseName={};",
                db_source.host, db_source.port, db_source.database_name,
            );
            let mut conn = MSSQL::new()?;

            match conn.connect(
                &db_url,
                db_source.user.as_str(),
                password.as_str(),
            ) {
                Ok(_) => {}
                Err(err) => {
                    warn!("数据库加密连接失败！尝试使用未加密连接: {err}");
                    let db_url = format!(
                        "jdbc:sqlserver://{}:{};DatabaseName={};trustServerCertificate=true",
                        db_source.host, db_source.port, db_source.database_name,
                    );
                    conn.connect(
                        &db_url,
                        db_source.user.as_str(),
                        password.as_str(),
                    )
                        .map_err(|err| anyhow!("数据库连接失败！: {err}"))?;
                }
            }

            let data = conn
                .execute_query(&query_sql)
                .map_err(|err| anyhow!("数据库查询失败！: {err}"))?;
            Ok(data)
        }
        Database::ORACLE => {
            let db_url = format!(
                "jdbc:oracle:thin:@//{}:{}/{}",
                db_source.host, db_source.port, db_source.database_name,
            );
            let mut conn = Oracle::new()?;

            conn.connect(
                &db_url,
                db_source.user.as_str(),
                password.as_str(),
            )
            .map_err(|err| anyhow!("数据库连接失败！: {err}"))?;

            let data = conn
                .execute_query(&query_sql)
                .map_err(|err| anyhow!("数据库查询失败！: {err}"))?;
            Ok(data)
        }
    }
}

fn decode_db_password(password: &String) -> String {
    let password = {
        // 加密过程查看crates/process_web/ui/lib/encrypt.ts
        let a = BASE64_STANDARD.decode(password).unwrap_or_default();
        let b = String::from_utf8(a).unwrap_or_default();

        match b.contains("DpSALt") {
            true => {
                let c = &b[6..];
                let d = BASE64_STANDARD.decode(c).unwrap_or_default();
                String::from_utf8(d).unwrap_or_default()
            }
            false => {
                password.clone()
            }
        }
    };

    password
}

#[async_trait]
impl Receive<DbConfig, Result<Db>> for Db {
    async fn receive(&mut self, query_sql: String, parameters: DbConfig) -> Result<Db> {
        let data = find_all_sql(&parameters.db_source_config, query_sql).await?;

        println!("data {data:?}");
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
    type Target = Result<Vec<String>>;

    async fn export(&mut self) -> Self::Target {
        if self.data.is_none() {
            return Err(anyhow!("self.data 中没有数据"));
        }
        let data = self.data.as_ref().unwrap();

        let template_sql = self
            .template_string
            .as_ref()
            .ok_or(anyhow!("未设置template_string"))?;

        if let Some(index) = template_sql.to_lowercase().find("insert into") {
            if index != 0 {
                return Err(anyhow!("这条语句不是插入语句！"));
            }
        } else {
            return Err(anyhow!("这条语句不是插入语句！"));
        }
        let sql_list = generate_sql_list(template_sql, data)?;

        if let Some(db_source) = &self.target_db_source_config {
            execute_sql(&db_source, sql_list.clone()).await?;
        }
        Ok(sql_list)
    }
}

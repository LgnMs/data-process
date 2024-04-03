use sea_orm::{ConnectionTrait, DatabaseBackend, DbConn, DbErr, Statement};

pub struct TableService;

impl TableService {
    pub async fn table_exists(db: &DbConn, database_name: &str, table_name: &str) -> Result<bool, DbErr> {
        let query = match db.get_database_backend() {
            DatabaseBackend::MySql => {
                format!("SELECT EXISTS (
                    SELECT 1
                    FROM information_schema.tables 
                    WHERE table_schema = '{database_name}'
                    AND table_name = '{table_name}'
                ) AS table_exists;")
            },
            DatabaseBackend::Postgres => {
                format!("SELECT EXISTS (
                    SELECT 1
                    FROM information_schema.tables 
                    WHERE table_catalog = '{database_name}' 
                    AND table_name = '{table_name}'
                ) AS table_exists;")
            },
            _ => {
                return Err(DbErr::Custom("不支持的数据库格式".to_owned()));
            },
        };

        let res = db
                .query_one(Statement::from_string(
                    db.get_database_backend(),
                    query,
                ))
                .await?
                .ok_or(DbErr::Custom("查询失败".to_owned()))?;

        res.try_get_by_index::<bool>(0)
        
    }
}

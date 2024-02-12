use anyhow::Result;

pub trait JDBC {
    type Connection;
    fn connect(&mut self, db_url: &str) -> Result<&Self::Connection>;

    fn create_statement(&mut self) -> Result<&Self::Connection>;

    fn prepare_statement(&mut self, sql_str: &str) -> Result<&Self::Connection>;

    fn close(&mut self) -> Result<()>;
}

pub trait ExecuteJDBC {
    type R;

    fn execute_query<T: From<Self::R>>(&mut self, query_str: &str) -> Result<Vec<T>>;

    fn execute_update(&mut self, query_str: &str) -> Result<()>;
}

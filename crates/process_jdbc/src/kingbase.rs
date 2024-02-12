use std::collections::HashMap;
use std::path::PathBuf;
use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder};
use anyhow::Result;
use crate::common::{ExecuteJDBC, JDBC};

pub struct Kingbase {
    pub jvm: Jvm,
    driver_path: PathBuf,
    conn: Option<Instance>,
    pub statement: Option<Instance>,
}

impl Kingbase {
    pub fn new() -> Result<Self> {
        let driver_path = "lib/kingbase8-8.6.0.jar";
        let entry = ClasspathEntry::new("lib/kingbase8-8.6.0.jar");
        let jvm= JvmBuilder::new()
            .classpath_entry(entry)
            .build()?;

         Ok(
             Self {
                jvm,
                driver_path: PathBuf::from(driver_path),
                conn: None,
                statement: None,
            }
         )
    }
}

impl JDBC for Kingbase {
    type Connection = Kingbase;

    fn connect(&mut self, jdbc_url: &str) -> Result<&Self::Connection> {
        let jdbc_str_arg = InvocationArg::try_from(jdbc_url)?;
        let conn = self.jvm.invoke_static(
            "java.sql.DriverManager",     // The Java class to create an instance for
            "getConnection",
            &vec![jdbc_str_arg],            // The `InvocationArg`s to use for the constructor call - empty for this example
        )?;
        self.conn = Some(conn);

        Ok(self)
    }

    fn create_statement(&mut self) -> Result<&Self::Connection> {
        let st = self.jvm.invoke(
            self.conn.as_ref().unwrap(),
            "createStatement",
            &Vec::new(),
        )?;

        self.statement = Some(st);

        Ok(self)
    }

    fn prepare_statement(&mut self, sql_str: &str) -> Result<&Self::Connection> {
        let st = self.jvm.invoke(
            self.conn.as_ref().unwrap(),
            "prepareStatement",
            &vec![InvocationArg::try_from(sql_str)?],
        )?;

        self.statement = Some(st);

        Ok(self)
    }

    fn close(&mut self) -> Result<()> {
        self.jvm.invoke(&self.statement.as_ref().unwrap(), "close", &Vec::new())?;

        self.statement = None;

        Ok(())
    }
}

impl ExecuteJDBC for Kingbase {
    type R = Vec<(String, String)>;
    fn execute_query<T: From<Self::R>>(&mut self, query_str: &str) -> Result<Vec<T>> {
        self.create_statement()?;

        let query_arg = InvocationArg::try_from(query_str)?;

        let rs = self.jvm.invoke(
            &self.statement.as_ref().unwrap(),
            "executeQuery",
            &vec![query_arg],
        )?;

        let mut map = HashMap::new();
        map.insert("name", "String");

        let mut vec = vec![];
        loop {
            let next = self.jvm.invoke(&rs, "next", &Vec::new())?;
            let bool_rust: bool = self.jvm.to_rust(next)?;
            if !bool_rust {
                break;
            }
            let mut temp_vec = vec![];
            for (key, value_type) in map.clone() {
                let value = match value_type {
                    "String" => {
                        Some(self.jvm.invoke(&rs, "getString", &vec![InvocationArg::try_from(key)?])?)

                    },
                    "i64" => {
                        Some(self.jvm.invoke(&rs, "getInter", &vec![InvocationArg::try_from(key)?])?)

                    },
                    _ => {
                        None
                    }
                };
                if let Some(val) = value {
                    let value_s: String = self.jvm.to_rust(val)?;
                    temp_vec.push((key.to_string(), value_s));
                }
            }
            vec.push(temp_vec)

        }
        self.close()?;
        let res = vec.iter().map(|x| {
            x.clone().into()
        }).collect::<Vec<T>>();

        Ok(res)
    }

    fn execute_update(&mut self, query_str: &str) -> Result<()> {
        self.prepare_statement(query_str)?;

        let rs = self.jvm.invoke(
            &self.statement.as_ref().unwrap(),
            "executeUpdate",
            &vec![],
        )?;

        self.close()?;
        Ok(())
    }
}
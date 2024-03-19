// J4RS_CONSOLE_LOG_LEVEL=debug

use log::debug;
#[macro_export]
macro_rules! impl_jdbc {
    ($struct: ty) => {
        impl $crate::common::JDBC for $struct {
            type Connection = $struct;

            fn connect(
                &mut self,
                jdbc_url: &str,
                username: &str,
                password: &str,
            ) -> Result<&Self::Connection> {
                debug!("jdbc url: {}", jdbc_url);
                let jdbc_str_arg = InvocationArg::try_from(jdbc_url)?;
                let username = InvocationArg::try_from(username)?;
                let password = InvocationArg::try_from(password)?;
                let conn = self.jvm.invoke_static(
                    "java.sql.DriverManager", // The Java class to create an instance for
                    "getConnection",
                    &vec![jdbc_str_arg, username, password], // The `InvocationArg`s to use for the constructor call - empty for this example
                )?;
                self.conn = Some(conn);

                Ok(self)
            }

            fn create_statement(&mut self) -> Result<&Self::Connection> {
                let st =
                    self.jvm
                        .invoke(self.conn.as_ref().unwrap(), "createStatement", &Vec::new())?;

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
                self.jvm
                    .invoke(&self.statement.as_ref().unwrap(), "close", &Vec::new())?;

                self.statement = None;

                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_execute_jdbc {
    ($struct: ty) => {
        use chrono::TimeZone;
        use j4rs::InvocationArg;
        use log::debug;
        use serde_json::{json, Value};
        use $crate::common::{ExecuteJDBC, JDBC};

        impl ExecuteJDBC for $struct {
            type R = Value;
            fn execute_query(&mut self, query_str: &str) -> Result<Vec<Self::R>> {
                debug!("{}", query_str);
                self.create_statement()?;

                let query_arg = InvocationArg::try_from(query_str)?;

                let rs = self.jvm.invoke(
                    &self.statement.as_ref().unwrap(),
                    "executeQuery",
                    &vec![query_arg],
                )?;

                let meta_data = self.jvm.invoke(&rs, "getMetaData", &vec![])?;

                let column_count_instance =
                    self.jvm.invoke(&meta_data, "getColumnCount", &vec![])?;
                let column_count: i32 = self.jvm.to_rust(column_count_instance)?;

                let mut vec = vec![];
                loop {
                    let next = self.jvm.invoke(&rs, "next", &Vec::new())?;
                    let bool_rust: bool = self.jvm.to_rust(next)?;
                    if !bool_rust {
                        break;
                    }
                    let mut map = serde_json::Map::new();
                    for i in 0..column_count {
                        let i = i + 1;
                        let column_name: String = self.jvm.to_rust(self.jvm.invoke(
                            &meta_data,
                            "getColumnName",
                            &[InvocationArg::try_from(i)?.into_primitive()?],
                        )?)?;

                        let column_type: i32 = self.jvm.to_rust(self.jvm.invoke(
                            &meta_data,
                            "getColumnType",
                            &vec![InvocationArg::try_from(i)?.into_primitive()?],
                        )?)?;

                        let value: Value = match $crate::common::JdbcType::from_i32(column_type) {
                            None => Value::Null,
                            Some(col_type) => match col_type {
                                $crate::common::JdbcType::Varchar => {
                                    let r = self.jvm.invoke(
                                        &rs,
                                        "getString",
                                        &vec![InvocationArg::try_from(i)?.into_primitive()?],
                                    )?;
                                    match self.jvm.to_rust(r) {
                                        Ok(x) => Value::String(x),
                                        Err(_) => Value::Null,
                                    }
                                }
                                $crate::common::JdbcType::Integer => {
                                    let r = self.jvm.invoke(
                                        &rs,
                                        "getInt",
                                        &vec![InvocationArg::try_from(i)?.into_primitive()?],
                                    )?;
                                    match self.jvm.to_rust(r) {
                                        Ok(x) => Value::Number(x),
                                        Err(_) => Value::Null,
                                    }
                                }
                                $crate::common::JdbcType::Float => {
                                    let r = self.jvm.invoke(
                                        &rs,
                                        "getFloat",
                                        &vec![InvocationArg::try_from(i)?.into_primitive()?],
                                    )?;
                                    match self.jvm.to_rust(r) {
                                        Ok(x) => Value::Number(x),
                                        Err(_) => Value::Null,
                                    }
                                }
                                $crate::common::JdbcType::Double => {
                                    let r = self.jvm.invoke(
                                        &rs,
                                        "getDouble",
                                        &vec![InvocationArg::try_from(i)?.into_primitive()?],
                                    )?;
                                    match self.jvm.to_rust(r) {
                                        Ok(x) => Value::Number(x),
                                        Err(_) => Value::Null,
                                    }
                                }
                                $crate::common::JdbcType::BigInt => {
                                    let r = self.jvm.invoke(
                                        &rs,
                                        "getLong",
                                        &vec![InvocationArg::try_from(i)?.into_primitive()?],
                                    )?;
                                    match self.jvm.to_rust(r) {
                                        Ok(x) => Value::Number(x),
                                        Err(_) => Value::Null,
                                    }
                                }
                                $crate::common::JdbcType::Decimal => {
                                    let r = self.jvm.invoke(
                                        &rs,
                                        "getDecimal",
                                        &vec![InvocationArg::try_from(i)?.into_primitive()?],
                                    )?;
                                    match self.jvm.to_rust(r) {
                                        Ok(x) => Value::Number(x),
                                        Err(_) => Value::Null,
                                    }
                                }
                                $crate::common::JdbcType::Boolean => {
                                    let r = self.jvm.invoke(
                                        &rs,
                                        "getBoolean",
                                        &vec![InvocationArg::try_from(i)?.into_primitive()?],
                                    )?;
                                    match self.jvm.to_rust(r) {
                                        Ok(x) => Value::Bool(x),
                                        Err(_) => Value::Null,
                                    }
                                }
                                $crate::common::JdbcType::Blob => {
                                    let r = self.jvm.invoke(
                                        &rs,
                                        "getBlob",
                                        &vec![InvocationArg::try_from(i)?.into_primitive()?],
                                    )?;
                                    match self.jvm.to_rust(r) {
                                        Ok(x) => Value::Object(x),
                                        Err(_) => Value::Null,
                                    }
                                }
                                $crate::common::JdbcType::Time => {
                                    let r = self.jvm.invoke(
                                        &rs,
                                        "getTime",
                                        &vec![InvocationArg::try_from(i)?.into_primitive()?],
                                    )?;
                                    match self.jvm.to_rust(r) {
                                        Ok(x) => Value::String(x),
                                        Err(err) => {
                                            println!("err {err}");
                                            Value::Null
                                        }
                                    }
                                }
                                $crate::common::JdbcType::Timestamp => {
                                    let r = self.jvm.invoke(
                                        &rs,
                                        "getTimestamp",
                                        &vec![InvocationArg::try_from(i)?.into_primitive()?],
                                    )?;
                                    match self.jvm.to_rust::<i64>(r) {
                                        Ok(x) => {
                                            let s = x / 1000;
                                            let ns = x as u32 % 1000;
                                            let dt = ::chrono::Local.timestamp_opt(s, ns).unwrap();
                                            let date_string =
                                                dt.format("%Y-%m-%d %H:%M:%S").to_string();
                                            Value::String(date_string)
                                        }
                                        Err(err) => {
                                            println!("err {err}");
                                            Value::Null
                                        }
                                    }
                                }
                                $crate::common::JdbcType::Null => Value::Null,
                                _ => {
                                    let r = self.jvm.invoke(
                                        &rs,
                                        "getString",
                                        &vec![InvocationArg::try_from(i)?.into_primitive()?],
                                    )?;
                                    match self.jvm.to_rust(r) {
                                        Ok(x) => Value::String(x),
                                        Err(_) => Value::Null,
                                    }
                                }
                            },
                        };
                        map.insert(column_name, value);
                    }
                    vec.push(json!(map));
                }
                self.jvm.invoke(&rs, "close", &vec![])?;
                self.close()?;
                Ok(vec)
            }

            fn execute_update(&mut self, query_str: &str) -> Result<()> {
                debug!("{}", query_str);
                self.prepare_statement(query_str)?;

                self.jvm
                    .invoke(&self.statement.as_ref().unwrap(), "executeUpdate", &vec![])?;

                self.close()?;
                Ok(())
            }
        }
    };
}

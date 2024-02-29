use anyhow::Result;
use j4rs::{ClasspathEntry, Instance, Jvm, JvmBuilder};
use crate::{impl_execute_jdbc, impl_jdbc};

pub struct MSSQL {
    pub jvm: Jvm,
    conn: Option<Instance>,
    pub statement: Option<Instance>,
}

impl MSSQL {
    pub fn new() -> Result<Self> {
        let entry = ClasspathEntry::new("libs/mssql-jdbc-12.6.1.jre11.jar");
        let jvm = JvmBuilder::new().classpath_entry(entry).build()?;

        Ok(Self {
            jvm,
            conn: None,
            statement: None,
        })
    }
}

impl_jdbc!(MSSQL);
impl_execute_jdbc!(MSSQL);

use crate::common::get_jvm;
use crate::{impl_execute_jdbc, impl_jdbc};
use anyhow::Result;
use j4rs::{Instance, Jvm};

pub struct MSSQL {
    pub jvm: Jvm,
    conn: Option<Instance>,
    pub statement: Option<Instance>,
}

impl MSSQL {
    pub fn new() -> Result<Self> {
        let jvm = get_jvm()?;
        Ok(Self {
            jvm,
            conn: None,
            statement: None,
        })
    }
}

impl_jdbc!(MSSQL);
impl_execute_jdbc!(MSSQL);

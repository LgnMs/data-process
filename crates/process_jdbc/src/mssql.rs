use std::sync::Arc;
use std::time::Instant;
use anyhow::Result;
use j4rs::{ClasspathEntry, Instance, Jvm, JvmBuilder};
use crate::{impl_execute_jdbc, impl_jdbc};
use crate::common::{get_jvm, jvm_is_setup, JvmInstance};

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

use anyhow::Result;
use j4rs::{ClasspathEntry, Instance, Jvm, JvmBuilder};
use crate::{impl_execute_jdbc, impl_jdbc};
use crate::common::get_jvm;

pub struct Oracle {
    pub jvm: Jvm,
    conn: Option<Instance>,
    pub statement: Option<Instance>,
}

impl Oracle {
    pub fn new() -> Result<Self> {
        let jvm = get_jvm()?;
        
        Ok(Self {
            jvm,
            conn: None,
            statement: None,
        })
    }
}

impl_jdbc!(Oracle);
impl_execute_jdbc!(Oracle);

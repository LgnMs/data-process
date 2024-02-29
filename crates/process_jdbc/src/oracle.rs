use anyhow::Result;
use j4rs::{ClasspathEntry, Instance, Jvm, JvmBuilder};
use crate::{impl_execute_jdbc, impl_jdbc};

pub struct Oracle {
    pub jvm: Jvm,
    conn: Option<Instance>,
    pub statement: Option<Instance>,
}

impl Oracle {
    pub fn new() -> Result<Self> {
        let entry = ClasspathEntry::new("libs/ojdbc10-19.22.0.0.jar");
        let jvm = JvmBuilder::new().classpath_entry(entry).build()?;

        Ok(Self {
            jvm,
            conn: None,
            statement: None,
        })
    }
}

impl_jdbc!(Oracle);
impl_execute_jdbc!(Oracle);

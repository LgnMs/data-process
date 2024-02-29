use anyhow::Result;
use j4rs::{ClasspathEntry, Instance, Jvm, JvmBuilder};
use crate::{impl_execute_jdbc, impl_jdbc};

pub struct Kingbase {
    pub jvm: Jvm,
    conn: Option<Instance>,
    pub statement: Option<Instance>,
}

impl Kingbase {
    pub fn new() -> Result<Self> {
        let entry = ClasspathEntry::new("libs/kingbase8-8.6.0.jar");
        let jvm = JvmBuilder::new().classpath_entry(entry).build()?;

        Ok(Self {
            jvm,
            conn: None,
            statement: None,
        })
    }
}

impl_jdbc!(Kingbase);
impl_execute_jdbc!(Kingbase);

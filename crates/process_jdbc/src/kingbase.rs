use crate::common::get_jvm;
use crate::{impl_execute_jdbc, impl_jdbc};
use anyhow::Result;
use j4rs::{Instance, Jvm};

pub struct Kingbase {
    pub jvm: Jvm,
    conn: Option<Instance>,
    pub statement: Option<Instance>,
}

impl Kingbase {
    pub fn new() -> Result<Self> {
        let jvm = get_jvm()?;
        Ok(Self {
            jvm,
            conn: None,
            statement: None,
        })
    }
}

impl_jdbc!(Kingbase);
impl_execute_jdbc!(Kingbase);

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

    fn execute_query(&mut self, query_str: &str) -> Result<Vec<Self::R>>;

    fn execute_update(&mut self, query_str: &str) -> Result<()>;
}

#[derive(Debug)]
pub enum JdbcType {
    Array = 2003,
    BigInt = -5,
    Binary = -2,
    Bit = -7,
    Blob = 2004,
    Boolean = 16,
    Char = 1,
    Clob = 2005,
    Datalink = 70,
    Date = 91,
    Decimal = 3,
    Double = 8,
    Float = 6,
    Integer = 4,
    LongNVarchar = -16,
    LongVarbinary = -4,
    LongVarchar = -1,
    NChar = -15,
    NClob = 2011,
    Null = 0,
    Numeric = 2,
    NVarchar = -9,
    Other = 1111,
    Real = 7,
    Ref = 2006,
    Rowid = -8,
    Smallint = 5,
    SqlXml = 2009,
    Struct = 2002,
    Time = 92,
    Timestamp = 93,
    Tinyint = -6,
    Varbinary = -3,
    Varchar = 12,
}

impl JdbcType {
    pub fn from_i32(value: i32) -> Option<JdbcType> {
        match value {
            2003 => Some(JdbcType::Array),
            -5 => Some(JdbcType::BigInt),
            -2 => Some(JdbcType::Binary),
            -7 => Some(JdbcType::Bit),
            2004 => Some(JdbcType::Blob),
            16 => Some(JdbcType::Boolean),
            1 => Some(JdbcType::Char),
            2005 => Some(JdbcType::Clob),
            70 => Some(JdbcType::Datalink),
            91 => Some(JdbcType::Date),
            3 => Some(JdbcType::Decimal),
            8 => Some(JdbcType::Double),
            6 => Some(JdbcType::Float),
            4 => Some(JdbcType::Integer),
            -16 => Some(JdbcType::LongNVarchar),
            -4 => Some(JdbcType::LongVarbinary),
            -1 => Some(JdbcType::LongVarchar),
            -15 => Some(JdbcType::NChar),
            2011 => Some(JdbcType::NClob),
            0 => Some(JdbcType::Null),
            2 => Some(JdbcType::Numeric),
            -9 => Some(JdbcType::NVarchar),
            1111 => Some(JdbcType::Other),
            7 => Some(JdbcType::Real),
            2006 => Some(JdbcType::Ref),
            -8 => Some(JdbcType::Rowid),
            5 => Some(JdbcType::Smallint),
            2009 => Some(JdbcType::SqlXml),
            2002 => Some(JdbcType::Struct),
            92 => Some(JdbcType::Time),
            93 => Some(JdbcType::Timestamp),
            -6 => Some(JdbcType::Tinyint),
            -3 => Some(JdbcType::Varbinary),
            12 => Some(JdbcType::Varchar),
            _ => None,
        }
    }
}
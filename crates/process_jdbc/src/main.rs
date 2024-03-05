use std::thread::sleep;
use std::time;
use std::time::Instant;
use process_jdbc::common::{ExecuteJDBC, JDBC};
// use process_jdbc::kingbase::Kingbase;
use process_jdbc::mssql::MSSQL;
// use process_jdbc::oracle::Oracle;
// 
fn main() {
//     // 人大金仓
//     // let mut conn = Kingbase::new().unwrap();
//     //
//     // conn.connect("jdbc:kingbase8://127.0.0.1:54321/test", "system", "123456")
//     //     .unwrap();
//     //
//     // // SELECT
//     // let c = conn
//     //     .execute_query(r##"SELECT * FROM public.test;"##)
//     //     .unwrap();
//     // println!("{:?}", c);
// 
// 
// 
//     // INSERT
//     // conn.execute_update(r#"INSERT INTO "public"."test" (name) VALUES('ttd');"#)
//     //     .expect("");
// 
// 
//     SQL SERVER
//     let mut conn2 = MSSQL::new().unwrap();

    // let start = Instant::now();
    // conn2.connect("jdbc:sqlserver://192.168.10.71:1433;databaseName=gzdata;Encrypt=false", "gzdata_user", "1a2s3d4f")
    //     .unwrap();
    // let duration = start.elapsed();
    // println!("jdbc执行时间: {:?}", duration);
    // JvmInstance::new().expect("TODO: panic message");

// 
//     
//     // ORACLE
//     // let mut conn3 = Oracle::new().unwrap();
//     // // jdbc:oracle:thin:@//<host>[:<port>]/<service_name>
//     // conn3.connect("jdbc:oracle:thin:@//127.0.0.1:1521/database", "user", "123456")
//     //     .unwrap();
//     //
//     // // SELECT
//     // let c = conn3
//     //     .execute_query(r##"select * from user_tables;"##)
//     //     .unwrap();
//     // println!("{:?}", c);
}

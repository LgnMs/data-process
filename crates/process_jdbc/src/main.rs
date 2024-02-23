use process_jdbc::common::{ExecuteJDBC, JDBC};
use process_jdbc::kingbase::Kingbase;


fn main() {
    let mut conn = Kingbase::new().unwrap();

    conn.connect("jdbc:kingbase8://192.168.40.3:54321/test?user=system&password=123456")
        .unwrap();

    // SELECT
    let c = conn.execute_query(r##"SELECT * FROM public.test;"##).unwrap();
    println!("{:?}", c);

    // INSERT
    // conn.execute_update(r#"INSERT INTO "public"."test" (name) VALUES('ttd');"#)
    //     .expect("TODO: panic message");

    // let res: Vec<Model> = conn.execute_query(r##"SELECT * FROM public.test;"##);
}

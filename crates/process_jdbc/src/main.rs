use j4rs::InvocationArg;
use process_jdbc::common::{ExecuteJDBC, JDBC};
use process_jdbc::kingbase::Kingbase;

#[derive(Debug)]
struct Model {
    name: String,
}

impl Model {
    fn execute_query(conn: &mut Kingbase, query_str: &str) -> anyhow::Result<Vec<Model>> {
        conn.create_statement()?;

        let query_arg = InvocationArg::try_from(query_str)?;

        let rs = conn.jvm.invoke(
            &conn.statement.as_ref().unwrap(),
            "executeQuery",
            &vec![query_arg],
        )?;

        let mut vec = vec![];
        loop {
            let next = conn.jvm.invoke(&rs, "next", &Vec::new())?;
            let bool_rust: bool = conn.jvm.to_rust(next)?;
            if !bool_rust {
                break;
            }
            let name =
                conn.jvm
                    .invoke(&rs, "getString", &vec![InvocationArg::try_from("name")?])?;
            let name_s: String = conn.jvm.to_rust(name)?;
            vec.push(Model { name: name_s })
        }
        conn.close()?;
        Ok(vec)
    }
}

// impl From<Vec<Vec<(String, String)>>> for Model {
//     fn from(value: Vec<Vec<(String, String)>>) -> Self {
//         let data = Model { name: "".to_string() };
//
//         for item in value {
//             let key =
//         }
//     }
// }

fn main() {
    let mut conn = Kingbase::new().unwrap();

    conn.connect("jdbc:kingbase8://192.168.40.3:54321/test?user=system&password=123456")
        .unwrap();

    // SELECT
    let c = Model::execute_query(&mut conn, r##"SELECT * FROM public.test;"##).unwrap();
    println!("{:?}", c);

    // INSERT
    conn.execute_update(r#"INSERT INTO "public"."test" (name) VALUES('ttd');"#)
        .expect("TODO: panic message");

    // let res: Vec<Model> = conn.execute_query(r##"SELECT * FROM public.test;"##);
}

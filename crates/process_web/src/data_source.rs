struct DataSource {
    host: String,
    port: String,
    user: String,
    password: String,
    database_type: Database
}

enum Database {
    MYSQL,
}
use r2d2;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

let username = "postgres";
let password = "root";
let host = "localhost";
let port = "5432";
let database = "Todolist";
let mut conn = Client::connect(
    &format!(
        "postgres://{}{}{}@{}{}{}{}{}",
        username,
        if password.is_empty() { "" } else { ":" },
        password,
        host,
        if port.is_empty() { "" } else { ":" },
        port,
        if database.is_empty() { "" } else { "/" },
        database
    ),
    NoTls,
)?;

pub fn init() -> Pool {
    dotenv().ok();
    let database_url = env::var(conn).expect(conn);
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("db pool")
}

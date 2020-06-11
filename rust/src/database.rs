use mysql::prelude::*;
use mysql::{self, Row};

pub struct Database {
    hostname: String,
    port: u16,
    user: String,
    database: String,
}

impl Database {
    pub fn new(hostname: &str, port: u16, user: &str, database: &str) -> Database {
        Database {
            hostname: hostname.to_string(),
            port: port,
            user: user.to_string(),
            database: database.to_string(),
        }
    }

    pub fn query(&self, query: &str, password: &str) -> Result<Vec<Row>, String> {
        let conn_options = mysql::OptsBuilder::new()
            .ip_or_hostname(Some(&self.hostname))
            .tcp_port(self.port)
            .user(Some(&self.user))
            .pass(Some(password))
            .db_name(Some(&self.database));
        let mut conn = mysql::Conn::new(conn_options).map_err(|x| format!("{:?}", x))?;
        conn.query(query)
            .map(|x: Vec<Row>| x)
            .map_err(|x| format!("{:?}", x))
    }
}

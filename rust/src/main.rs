mod database;
mod encryptonize;
mod protected_database;
mod storage;

use crate::encryptonize::Encryptonize;
use crate::protected_database::ProtectedDatabase;
use crate::storage::Storage;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Protected Database",
    about = "A database with access protected by Encryptonize."
)]
struct Options {
    /// MySQL hostname
    #[structopt(short, long, conflicts_with = "init", required_unless = "init")]
    host: Option<String>,

    /// MySQL port
    #[structopt(short, long, conflicts_with = "init", required_unless = "init")]
    port: Option<u16>,

    /// MySQL username
    #[structopt(short, long, conflicts_with = "init", required_unless = "init")]
    user: Option<String>,

    /// Database name
    #[structopt(short, long, conflicts_with = "init", required_unless = "init")]
    database: Option<String>,

    /// Encryptonize user token
    #[structopt(short, long)]
    token: String,

    /// MySQL query to perform
    #[structopt(short, long, conflicts_with = "init", required_unless = "init")]
    query: Option<String>,

    /// Store the database password
    #[structopt(short, long)]
    init: Option<String>,
}

fn main() -> Result<(), String> {
    let options = Options::from_args();

    match options.init {
        Some(password) => {
            let encryptonize = Encryptonize::new(&options.token);
            let storage = Storage::new("db_pass.enc");

            // Encrypt password using Encryptonize
            let encrypted_password = encryptonize.encrypt(password.to_string())?;

            // Store password
            storage
                .put(&encrypted_password)
                .map_err(|x| format!("{:?}", x))?;
        }
        None => {
            let database = ProtectedDatabase::new(
                &options.host.unwrap(),
                options.port.unwrap(),
                &options.user.unwrap(),
                &options.database.unwrap(),
                &options.token,
            );

            let result = database.query(&options.query.unwrap())?;

            println!("Query result:");
            for row in result {
                println!("{:?}", row);
            }
        }
    }

    Ok(())
}

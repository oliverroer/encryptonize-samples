mod database;
mod encryptonize;
mod protected_database;
mod storage;

use crate::encryptonize::Encryptonize;
use crate::protected_database::ProtectedDatabase;
use crate::storage::Storage;
use structopt::StructOpt;

/// Options to pass to the program.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Protected Database",
    about = "A database with access protected by Encryptonize."
)]
enum Options {
    /// Encrypt the database password and store it.
    #[structopt(name = "init")]
    Init {
        /// Encryptonize user token.
        #[structopt(short, long)]
        token: String,

        /// Encryptonize group ID.
        #[structopt(short, long)]
        group: Option<String>,

        /// Database password to store.
        #[structopt(short, long)]
        password: String,
    },

    /// Make a query to the database.
    #[structopt(name = "query")]
    Query {
        /// MySQL hostname.
        #[structopt(short, long)]
        host: Option<String>,

        /// MySQL port.
        #[structopt(short, long)]
        port: Option<u16>,

        /// MySQL username.
        #[structopt(short, long)]
        user: Option<String>,

        /// Database name.
        #[structopt(short, long)]
        database: Option<String>,

        /// Encryptonize user token.
        #[structopt(short, long)]
        token: String,

        /// MySQL query to perform.
        #[structopt(short, long)]
        query: Option<String>,
    },
}

fn main() -> Result<(), String> {
    let options = Options::from_args();

    match options {
        Options::Init {
            token,
            group,
            password,
        } => {
            let encryptonize = Encryptonize::new(&token, group.as_deref());
            let storage = Storage::new("db_pass.enc");

            // Encrypt password using Encryptonize
            let encrypted_password = encryptonize.encrypt(password.to_string())?;

            // Store password
            storage
                .put(&encrypted_password)
                .map_err(|x| format!("{:?}", x))?;
        }
        Options::Query {
            host,
            port,
            user,
            database,
            token,
            query,
        } => {
            let database = ProtectedDatabase::new(
                &host.unwrap(),
                port.unwrap(),
                &user.unwrap(),
                &database.unwrap(),
                &token,
            );

            let result = database.query(&query.unwrap())?;

            println!("Query result:");
            for row in result {
                println!("{:?}", row);
            }
        }
    }

    Ok(())
}

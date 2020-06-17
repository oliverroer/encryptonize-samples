//! A database with access protected by Encryptonize.
use crate::database::Database;
use crate::encryptonize::Encryptonize;
use crate::storage::Storage;
use mysql::Row;

/// Represents a connection to a database with access protected by Encryptonize.
pub struct ProtectedDatabase {
    database: Database,
    encryptonize: Encryptonize,
    storage: Storage,
}

impl ProtectedDatabase {
    /// Create a new connection to a database.
    ///
    /// # Arguments
    /// * `hostname` - Hostname/IP of the MySQL server.
    /// * `port` - TCP/IP port of the MySQL server.
    /// * `user` - Username when accessing the database.
    /// * `database` - Name of the database.
    /// * `user_token` - Encryptonize user token.
    pub fn new(
        hostname: &str,
        port: u16,
        user: &str,
        database: &str,
        user_token: &str,
    ) -> ProtectedDatabase {
        ProtectedDatabase {
            database: Database::new(hostname, port, user, database),
            encryptonize: Encryptonize::new(user_token, None),
            storage: Storage::new("db_pass.enc"),
        }
    }

    /// Make a query to the database.
    ///
    /// # Arguments
    /// * `query` - SQL query to perform.
    pub fn query(&self, query: &str) -> Result<Vec<Row>, String> {
        // Retrieve encrypted database password from storage
        let encrypted_password = self.storage.get().map_err(|x| format!("{:?}", x))?;

        // Decrypt password using Encryptonize
        let password = self.encryptonize.decrypt(encrypted_password)?;

        // Query the database with the decrypted password
        self.database.query(query, &password)
    }
}

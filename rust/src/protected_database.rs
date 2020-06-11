use crate::database::Database;
use crate::encryptonize::Encryptonize;
use crate::storage::Storage;
use mysql::Row;

pub struct ProtectedDatabase {
    database: Database,
    encryptonize: Encryptonize,
    storage: Storage,
}

impl ProtectedDatabase {
    pub fn new(
        hostname: &str,
        port: u16,
        user: &str,
        database: &str,
        user_token: &str,
    ) -> ProtectedDatabase {
        ProtectedDatabase {
            database: Database::new(hostname, port, user, database),
            encryptonize: Encryptonize::new(user_token),
            storage: Storage::new("db_pass.enc"),
        }
    }

    pub fn query(&self, query: &str) -> Result<Vec<Row>, String> {
        // Retrieve encrypted database password from storage
        let encrypted_password = self.storage.get().map_err(|x| format!("{:?}", x))?;

        // Decrypt password using Encryptonize
        let password = self.encryptonize.decrypt(encrypted_password)?;

        // Query the database with the decrypted password
        self.database.query(query, &password)
    }
}

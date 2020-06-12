# Encryptonize API Rust Example
This example implements a database connection that is protected by the Encryptonize API. The
application can be used to query an existing password protected database. The password is encrypted
by Encryptonize, and can therefore safely be placed in cloud storage where multiple Encryptonize
users can easily access it, without ever knowing the database password itself. Access to the
database can be restricted using the Group functionality of Encryptonize.

Note that for this simple example, the encrypted password is just stored locally. However, the
`storage` module could easily be modified to fetch the file from e.g. AWS S3.

## Build and Run

To build the application you need `cargo` installed. You can find instructions
[here](https://rustup.rs/). Then simply run `cargo build --release`. The resulting binary is located
at `target/release/protected-database`.

Run `protected-database --help` to see the possible parameters to pass to the application. To
generate code documentation, run `cargo doc --open`.

To set up the encrypted password, you can run
```bash
protected-database init -g <group_id> -t <user_token> -p <database_password>
```
where `user_token` is the users API token and `group_id` is an optional Group ID. The Group ID can
be provided to give a group of users access to the database. The API token and Group ID can be
obtained from the [Encryptonize Frontend](https://encryptonize.cyber-crypt.com).

Once the encrypted password has been created, you can query the database by running
```bash
protected-database query -t <user_token> -h <db_hostname> -p <db_port> -u <db_user> -d <db_name> -q <sql_query>
```

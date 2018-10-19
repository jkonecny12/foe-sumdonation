extern crate rusqlite;

use self::rusqlite::Connection;
use std::path::Path;
use db::{DBConnector,DBTableInitiator};


pub struct SQLiteDB {
    db_path: String,
    connection: Option<Connection>,
}

impl SQLiteDB {

    pub fn new(path: String) -> SQLiteDB {
        SQLiteDB {
            db_path: path,
            connection: None,
        }
    }

    fn get_connection(&self) -> &Connection {
         if let Some(ref con) = self.connection {
             return &con;
         } else {
             panic!("Connection wasn't created yet!");
         }
    }

    fn create_players_sql(&self) {
        self.get_connection().execute(
            "CREATE TABLE IF NOT EXISTS Players(
                id      INTEGER PRIMARY KEY,
                name    STRING,
                age     INTEGER,
                FOREIGN KEY(age) REFERENCES Age(id))", &[]).unwrap();
    }

    fn create_age_sql(&self) {
        self.get_connection().execute(
            "CREATE TABLE IF NOT EXISTS Age(
                id      INTEGER PRIMARY KEY,
                name    STRING)", &[]).unwrap();
    }

    fn create_resources_sql(&self) {
        self.get_connection().execute(
            "CREATE TABLE IF NOT EXISTS Resources(
                id      INTEGER PRIMARY KEY,
                name    STRING,
                age     INTEGER,
                FOREIGN KEY(age) REFERENCES Age(id))", &[]).unwrap();
    }

    fn create_donations_sql(&self) {
        self.get_connection().execute(
            "CREATE TABLE IF NOT EXISTS Donations(
                id      INTEGER PRIMARY KEY,
                who     INTEGER,
                what    INTEGER,
                count   INTEGER,
                FOREIGN KEY(who) REFERENCES Players(id),
                FOREIGN KEY(what) REFERENCES Resources(id))", &[]).unwrap();
    }
}

impl DBConnector for SQLiteDB {

    fn connect(&mut self) -> Result<(), String> {
        let path = Path::new(&self.db_path);
        let con = Connection::open(path);

        match con {
            Ok(connection) => {
                self.connection = Some(connection);
                return Ok(())
            },
            Err(err) => Err(err.to_string()),
        }

    }

}

impl DBTableInitiator for SQLiteDB {

    fn create_tables(&self) -> bool {
        self.create_age_sql();
        self.create_players_sql();
        self.create_resources_sql();
        self.create_donations_sql();
        true
    }
}

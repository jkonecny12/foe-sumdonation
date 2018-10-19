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

    fn create_donations_sql(&self) -> bool {
        let res = self.get_connection().execute(
                "CREATE TABLE IF NOT EXISTS Donations(
                    id      INTEGER PRIMARY KEY,
                    who     INTEGER,
                    what    INTEGER,
                    count   INTEGER)", &[]).unwrap();

       if res > 0 {
           return true
       } else {
           return false
       }
    }
}

impl DBConnector for SQLiteDB {

    fn connect(&mut self) -> Result<String, String> {
        let path = Path::new(&self.db_path);
        let con = Connection::open(path);

        match con {
            Ok(connection) => {
                self.connection = Some(connection);
                return Ok("".to_string())
            },
            Err(err) => Err(err.to_string()),
        }

    }

}

impl DBTableInitiator for SQLiteDB {

    fn create_tables(&self) -> bool {
        self.create_donations_sql()
    }
}

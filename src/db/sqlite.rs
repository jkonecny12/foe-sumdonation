extern crate rusqlite;

use self::rusqlite::Connection;
use std::path::Path;
use db::DBConnector;


pub struct SQLiteDB {
    pub db_path: String,
    pub connection: Option<Connection>,
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


impl SQLiteDB {

    pub fn new(path: String) -> SQLiteDB {
        SQLiteDB {
            db_path: path,
            connection: None,
        }
    }

}

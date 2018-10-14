/* Define traits required to work with the database */

pub mod sqlite;

pub trait DBConnector {
    fn connect(&mut self) -> Result<String, String>;
}

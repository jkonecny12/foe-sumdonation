/* Define traits required to work with the database */

pub mod sqlite;

pub trait DBConnector {
    fn connect(&mut self) -> Result<String, String>;
}

pub trait DBTableInitiator {
    fn create_tables(&self) -> bool;
}

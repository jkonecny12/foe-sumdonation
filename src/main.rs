mod db;

use db::sqlite::SQLiteDB;
use db::{DBConnector,DBInitiateTables};

fn main() {
    let db_path = "./test.sqlite".to_string();

    let mut connection = SQLiteDB::new(db_path);
    connection.connect().unwrap();
    let is_created = connection.create_tables();

    if is_created {
        println!("Is created")
    } else {
        println!("Not created")
    }

    println!("Success!")
}

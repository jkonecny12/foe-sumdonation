mod db;

use db::sqlite::SQLiteDB;
use db::DBConnector;

fn main() {
    let db_path = "./test.sqlite".to_string();

    let mut connection = SQLiteDB::new(db_path);
    connection.connect().unwrap();

    println!("Success!")
}

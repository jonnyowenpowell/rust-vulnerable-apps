/*
dependencies]
diesel = { version = "2.2.0", features = ["postgres", "sqlite"] }
*/

use diesel;
use diesel::result::QueryResult;
use diesel::prelude::*;
use diesel::dsl::sql_query;
use std::borrow::BorrowMut;
use std::fs;
use std::string::String;
use diesel::sql_types::Text;

// Based on https://github.com/diesel-rs/diesel/blob/2.1.x/examples/sqlite/getting_started_step_3/src/lib.rs

diesel::table! {
  some_table (id) {
      id    -> Integer,
      key   -> Text,
      notes -> Text,
  }
}


#[derive(QueryableByName, PartialEq, Debug)]
#[diesel(table_name = some_table)]
pub struct DbItem {
    id: i32,
    key: String, 
    notes: String,
}

fn main() {
  let mut conn = SqliteConnection::establish("/var/tmp/database.db")
        .unwrap_or_else(|_| panic!("Error connecting to db"));
  let db_key = fs::read_to_string("/var/tmp/tainted_file.txt").unwrap();
  // NOT VULNERABLE TO Sqli HERE
  let results: QueryResult<Vec<DbItem>> = sql_query("SELECT * FROM someTable WHERE key = ?")
    .bind::<Text, _>(db_key.trim())
    .load::<DbItem>(&mut conn);

  println!("Results: {:?} for key {:?}", results.unwrap(), db_key);
}

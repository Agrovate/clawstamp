use std::env::current_dir;

use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

const DB_URL: &str = "sqlite:///home/nishant/Projects/clawstamp/test";

#[tokio::main]
async fn main() {

    //Connection to SQLite
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database: ");
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Database created Successfully"),
            Err(e) => panic!("error: {}",e)
        }
    } 
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    


    // To get current directory name
    let loc = current_dir().unwrap().to_string_lossy().to_string();
    let dirs:Vec<&str> = loc.split("/").collect();
    let dir = dirs[dirs.len() - 1];
    
    sqlx::query!("insert into times (dir, start) values(?, ?)",dir, 1).execute(&db).await.unwrap();
}

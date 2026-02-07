use std::env::{self, current_dir};
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase, FromRow};
use dotenvy::{dotenv, from_path};
use chrono::{DateTime, Duration, Local};


#[derive(Debug, FromRow)]
pub struct Project {
   pub dir: String,
   pub start: DateTime<Local>,
   pub stop: Option<DateTime<Local>>,
   pub duration: i64 
}



#[tokio::main]
async fn main() {

    dotenv().ok();
    
    from_path("/home/nishant/Projects/clawstamp/.env").unwrap();
    let db_url = env::var("DATABASE_URL"). expect("DATABASE_URL must be set");
    
    //Connection to SQLite
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating database: ");
        match Sqlite::create_database(&db_url).await {
            Ok(_) => println!("Database created Successfully"),
            Err(e) => panic!("error: {}",e)
        }
    } 
    let db = SqlitePool::connect(&db_url).await.unwrap();
    


    // To get current directory name
    let loc = current_dir().unwrap().to_string_lossy().to_string();
    let dirs:Vec<&str> = loc.split("/").collect();
    let starts = Local::now();
    let stops =  starts + Duration::days(1) ;
    let p1 = Project {
         dir : dirs[dirs.len() - 1].to_string(),
         start: starts,
         stop: Some(stops),
         duration: (stops - starts).num_seconds()
    };

    sqlx::query!("insert into times(dir, start, duration) values($1, $2, $3)",
        p1.dir, p1.start, p1.duration
    ).execute(&db).await.unwrap();
    let data = sqlx::query_as::<_, Project>("select * from times;").fetch_all(&db).await.unwrap();
    
    for row in data {
        println!("{:?}", row);
    }
}

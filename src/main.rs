use std::env::{self, current_dir};
use sqlx::{FromRow, Row, Sqlite, SqlitePool, migrate::MigrateDatabase};
use dotenvy::{dotenv, from_path};
use chrono::{DateTime, Duration, Local};


#[derive(Debug, FromRow)]
pub struct Project {
   pub dir: String,
   pub start: DateTime<Local>,
   pub stop: Option<DateTime<Local>>,
   pub duration: Option<i64>
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
    let action = "show";
    match action {
       "start" => start(&db).await.unwrap(),
       "stop" => stop(&db).await.unwrap(),
       "show" => show(&db).await.unwrap(),
        _ => println!("shi is broken")
    }
}

async fn start(db: &SqlitePool) -> Result<(), sqlx::Error> {
    let loc = current_dir().unwrap().to_string_lossy().to_string();
    let dirs:Vec<&str> = loc.split("/").collect();
    let start_time = Local::now();

    let p = Project {
         dir : dirs[dirs.len() - 1].to_string(),
         start: start_time,
         stop: None,
         duration: None
    };

    sqlx::query!("insert into times(dir, start) values($1, $2)",
        p.dir, p.start
    ).execute(db).await.unwrap();

    Ok(())
}


async fn stop(db: &SqlitePool) -> Result<(), sqlx::Error> {
    let loc = current_dir().unwrap().to_string_lossy().to_string();
    let dirs:Vec<&str> = loc.split("/").collect();
    let stop_time = Local::now();
    let row = sqlx::query("select start from times where dir = $1")
        .bind(dirs[dirs.len() - 1].to_string())
        .fetch_one(db)
        .await?;

    let start_time: DateTime<Local> = row.try_get("start")?;

    let diff = stop_time - start_time; 
    sqlx::query("UPDATE times SET stop = $1, duration = $2 WHERE dir = $3")
        .bind(stop_time)
        .bind(diff.num_seconds())
        .bind(dirs[dirs.len() - 1].to_string())
        .execute(db)
        .await?;

    Ok(())
}

async fn show(db: &SqlitePool) -> Result<(), sqlx::Error> {
    let data = sqlx::query_as::<_,Project>("select * from times")
        .fetch_all(db).await?;
    println!("{:?}",data);

    Ok(())
}

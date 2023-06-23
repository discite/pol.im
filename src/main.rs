use std::sync::Arc;

use sqlx::{migrate::MigrateDatabase, Sqlite, sqlite::SqlitePoolOptions, FromRow, types::time::Date};

use axum::{
    http::StatusCode, routing::get, Router,
    extract::{TypedHeader},
    headers::UserAgent,
};

const DB_URL: &str = "sqlite://sqlite.db";
// the fields we'll be retrieving from an sql query

#[derive(FromRow, Debug, Clone)]
pub struct Post {
    pub post_title: String,
    pub post_date: Date,
    pub post_body: String,
}

async fn index(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    String::from(user_agent.as_str()) 
}

#[tokio::main]
async fn main() {
    // SQLx create table using rust code
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    // let pool = SqlitePool::connect(DB_URL).await.unwrap(); //For Testing
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(DB_URL).await.expect("Couldnt connect to the database");
    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS myposts (
            post_id INTEGER PRIMARY KEY NOT NULL, 
            post_date DATE NOT NULL DEFAULT CURRENT_DATE,
            post_title TEXT,
            post_body TEXT
        );"
    ).execute(&pool).await.unwrap();
    println!("Create myposts table result: {:?}", result);

    // Fetch all of the posts at the start of the program to avoid
    // hitting the db for each page request
    let posts = sqlx::query_as::<_, Post>("select post_title, post_date, post_body from myposts") 
        .fetch_all(&pool)
        .await
        .unwrap();

    // Above we retrieved Vec<Post> 
	// We place it in an Arc for thread-safe referencing.  
    let shared_state = Arc::new(posts);

    let app = Router::new()
        .route("/", get(index))
        .route("/post/:query_title", get(post))
        // We pass the shared state to our handlers 
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
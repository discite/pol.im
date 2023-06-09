use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use axum::{
    http::StatusCode, routing::get, Router,
    extract::{TypedHeader},
    headers::UserAgent,
};

const DB_URL: &str = "sqlite://sqlite.db";


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
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS myposts (
            post_id INTEGER PRIMARY KEY NOT NULL, 
            post_date DATE NOT NULL DEFAULT CURRENT_DATE,
            post_title TEXT,
            post_body TEXT
        );"
    ).execute(&db).await.unwrap();
    println!("Create myposts table result: {:?}", result);

    let app = Router::new()
        .route("/", get(index));

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
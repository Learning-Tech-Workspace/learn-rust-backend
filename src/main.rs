use std::env;

use axum::Extension;
use dotenv::dotenv;

pub mod enums;
pub mod features;

mod router;

use router::create_router;
use sea_orm::{ConnectOptions, Database};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    // load environment variables from a .env file
    dotenv().ok();

    let opt = ConnectOptions::new(env::var("DATABASE_URL").unwrap());

    let db_connection = match Database::connect(opt).await {
        Ok(conn) => conn,
        Err(e) => panic!("Error connecting to the database: {:?}", e),
    };

    // build our application with a route
    let app = create_router().layer(Extension(db_connection));

    println!("🚀 Starting server at http://localhost:3000");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

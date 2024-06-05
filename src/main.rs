use std::env;

use axum::{http::HeaderValue, Extension};
use dotenv::dotenv;

pub mod enums;
pub mod features;

mod error;
mod router;
mod socket;
mod utils;

use router::create_router;
use sea_orm::{ConnectOptions, Database};
use socketioxide::handler::ConnectHandler;
use socketioxide::SocketIo;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_http::{cors::CorsLayer, services::ServeDir};

use socket::{check_login, on_connect};

use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    // load environment variables from a .env file
    dotenv().ok();

    let opt = ConnectOptions::new(
        env::var("DATABASE_URL").map_err(|_| Error::EnvVarNotFound("DATABASE_URL".to_string()))?,
    );

    let db_connection = Database::connect(opt)
        .await
        .map_err(|_| Error::DatabaseConnectionFailed)?;

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect.with(check_login));

    // build our application with a route
    let app = create_router()
        .fallback_service(ServeDir::new("public"))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
                .layer(Extension(db_connection))
                .layer(layer),
        );

    println!("🚀 Starting server at http://localhost:3000");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

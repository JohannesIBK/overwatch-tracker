use std::net::SocketAddr;

use sea_orm::Database;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use migration::{Migrator, MigratorTrait};

use crate::configuration::app::AppState;
use crate::configuration::env;

mod api;
mod configuration;
mod ext;

#[tokio::main]
async fn main() {
    pid1::Pid1Settings::new()
        .enable_log(true)
        .launch()
        .expect("Launch failed");

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(filter).init();
    
    let database = Database::connect(*env::DATABASE_URL)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&database, None)
        .await
        .expect("Failed to run migrations");

    let app = axum::Router::new()
        .nest("/api/game", api::init_game_routes())
        .nest("/api/user", api::init_user_routes())
        .layer(configuration::get_cors_layer())
        .with_state(AppState { db: database });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to port");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}

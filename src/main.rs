mod db;
mod handlers;
mod routes;

use db::new;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let shared_state = new();

    //create our tcp listener
    // Run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Could not create tcp listener");
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    println!("listening on {}", listener.local_addr().unwrap());

    // compose the routes
    let app = routes::new().with_state(shared_state);

    //serve the application
    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}

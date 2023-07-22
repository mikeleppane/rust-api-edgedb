mod models;

mod api;
mod db;

use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_target(false).pretty().init();

    let Ok(db) = db::initialize_db().await else {
        panic!("Could not create EdgeDB client!");
    };

    let store = db::EdgeDBUserStore::new(db);

    let port = std::env::var("PORT")
        .expect("PORT must be set.")
        .to_string();

    let addr: std::net::SocketAddr = std::net::SocketAddr::from((
        [0, 0, 0, 0],
        port.parse::<u16>()
            .unwrap_or_else(|_| panic!("could not parse the given port {}", port)),
    ));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(api::build_router(store).into_make_service())
        .await
        .unwrap();
    Ok(())
}

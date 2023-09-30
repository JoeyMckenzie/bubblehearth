use std::sync::Arc;

use anyhow::Context;
use axum::Router;
use tracing::info;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use bubblehearth::client::BubbleHearthClient;
use bubblehearth::localization::Locale;
use bubblehearth::regionality::AccountRegion;

struct AppState {
    client: BubbleHearthClient,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("loading environment variables");

    dotenvy::dotenv().context("failed to load environment, make sure a .env file exists")?;

    let client_id = std::env::var("CLIENT_ID").context("client id not found in the environment")?;
    let client_secret =
        std::env::var("CLIENT_SECRET").context("client secret not found in the environment")?;

    info!("initializing application routes");

    let app_state = AppState {
        client: BubbleHearthClient::new(client_id, client_secret, AccountRegion::US, Locale::EnglishUS),
    };

    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let router = Router::new()
        //.route("/user/:profile", get(get_user))
        // .route("/item/:id", get(get_item))
        .with_state(Arc::new(app_state));

    info!("now listening on port {}", port);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

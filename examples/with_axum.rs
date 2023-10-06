use std::sync::Arc;

use anyhow::Context;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use http::StatusCode;
use serde::Serialize;
use thiserror::Error;
use tracing::info;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use bubblehearth::classic::realms::Realm;
use bubblehearth::client::BubbleHearthClient;
use bubblehearth::localization::{Locale, StringOrStructLocale};
use bubblehearth::regionality::AccountRegion;

struct AppState {
    client: BubbleHearthClient,
}

/// Define your own structs for marshalling between the client provided responses
/// and what you'd like to return to users. The client does not provide serialization
/// for responses, so it's up to consumers on how they want to represent the typed
/// responses provided by the library.
#[derive(Serialize)]
struct RealmResponse {
    name: String,
}

#[derive(Debug, Error)]
enum AppError {
    #[error("Realm {0} was not found.")]
    RealmNotFound(String),
    #[error("Uh oh... an error occurred :(")]
    InternalError,
    #[error("An error occurred calling the API.")]
    ClientError(#[from] bubblehearth::errors::BubbleHearthError),
}

impl TryFrom<Realm> for RealmResponse {
    type Error = AppError;

    fn try_from(value: Realm) -> Result<Self, Self::Error> {
        match value.name {
            StringOrStructLocale::StringLocale(realm) => Ok(RealmResponse { name: realm }),
            StringOrStructLocale::StructLocale(_) => Err(AppError::InternalError),
        }
    }
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
        client: BubbleHearthClient::new(
            client_id,
            client_secret,
            AccountRegion::US,
            Locale::EnglishUS,
        ),
    };

    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let router = Router::new()
        .route("/realm/:slug", get(get_realm))
        .with_state(Arc::new(app_state));

    info!("now listening on port {}", port);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

async fn get_realm(
    Path(realm): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<RealmResponse>, AppError> {
    let region = state.client.classic().get_realm(&realm).await?;

    match region {
        None => Err(AppError::RealmNotFound(realm)),
        Some(found_region) => {
            let response: Json<RealmResponse> = Json(found_region.try_into()?);
            Ok(response)
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::RealmNotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        status.into_response()
    }
}

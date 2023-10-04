use bubblehearth::client::BubbleHearthClient;
use bubblehearth::localization::Locale;
use bubblehearth::regionality::AccountRegion;

#[tokio::test]
async fn returns_access_token_given_proper_credentials() {
    // Arrange
    dotenvy::dotenv().expect("test client credentials unable to load");
    let client_id = std::env::var("CLIENT_ID").expect("test client ID not found");
    let client_secret = std::env::var("CLIENT_SECRET").expect("test client secret not found");
    let client = BubbleHearthClient::new(
        client_id,
        client_secret,
        AccountRegion::US,
        Locale::EnglishUS,
    );

    // Act
    let token = client.get_access_token().await;

    // Assert
    assert!(token.is_ok());
    assert!(!token.unwrap().is_empty());
}

#[tokio::test]
async fn returns_cached_access_token_when_multiple_calls_outgoing() {
    // Arrange
    dotenvy::dotenv().expect("test client credentials unable to load");
    let client_id = std::env::var("CLIENT_ID").expect("test client ID not found");
    let client_secret = std::env::var("CLIENT_SECRET").expect("test client secret not found");
    let client = BubbleHearthClient::new(
        client_id,
        client_secret,
        AccountRegion::US,
        Locale::EnglishUS,
    );

    // Act, get the first token and verify it's the cached token the second time around
    let token = client.get_access_token().await.unwrap();
    let cached_token = client.get_access_token().await.unwrap();

    // Assert
    assert_eq!(token, cached_token);
}

#[tokio::test]
async fn returns_error_when_credentials_invalid() {
    // Arrange
    dotenvy::dotenv().expect("test client credentials unable to load");
    let client = BubbleHearthClient::new(
        "client_id".to_string(),
        "client_secret".to_string(),
        AccountRegion::US,
        Locale::EnglishUS,
    );

    // Act
    let token = client.get_access_token().await;

    // Assert
    assert!(token.is_err());
}

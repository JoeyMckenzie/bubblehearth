use bubblehearth::client::BubbleHearthClient;
use bubblehearth::localization::Locale;
use bubblehearth::regionality::AccountRegion;

#[tokio::test]
async fn returns_realms_index() {
    // arrange
    dotenvy::dotenv().expect("test client credentials unable to load");
    let client_id = std::env::var("CLIENT_ID").expect("test client ID not found");
    let client_secret = std::env::var("CLIENT_SECRET").expect("test client secret not found");
    let client = BubbleHearthClient::new(
        client_id,
        client_secret,
        AccountRegion::US,
        Locale::EnglishUS,
    );

    // act
    let realms_index = client.classic.get_realms().await;

    // assert
    assert!(realms_index.is_ok());
    assert!(!realms_index.unwrap().realms.is_empty());
}

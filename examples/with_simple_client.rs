use bubblehearth::client::BubbleHearthClient;
use bubblehearth::localization::Locale;
use bubblehearth::regionality::AccountRegion;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("failed to load environment variables");

    let client_id = std::env::var("CLIENT_ID").expect("client ID not found within the environment");
    let client_secret =
        std::env::var("CLIENT_SECRET").expect("client secret not found within the environment");

    let client = BubbleHearthClient::new(
        client_id,
        client_secret,
        AccountRegion::US,
        Locale::EnglishUS,
    );

    // Get a list of World of Warcraft Classic realms
    let realms = client.classic().get_realms().await.unwrap();
    dbg!(realms);

    // Get an individual Classic realm
    let realm = client.classic().get_realm("westfall").await.unwrap();
    dbg!(realm);

    // We can explicitly request access tokens, though the client will internally grab one from Blizzard and refresh as needed
    // In the example above, we simply create a client connection and start sending requests
    let _token = client
        .get_access_token()
        .await
        .expect("access token was unable to be retrieved");
}

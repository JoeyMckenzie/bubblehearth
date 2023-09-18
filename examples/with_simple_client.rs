use bubblehearth::client::BubbleHearthClient;
use bubblehearth::regionality::AccountRegion;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("failed to load environment variables");

    let client_id = std::env::var("CLIENT_ID").expect("client ID not found within the environment");
    let client_secret =
        std::env::var("CLIENT_SECRET").expect("client secret not found within the environment");

    let mut client = BubbleHearthClient::new(client_id, client_secret, AccountRegion::US);
    let token = client
        .get_access_token()
        .await
        .expect("access token was unable to be retrieved");
    dbg!(token);

    let refresh_required = client.authentication.unwrap().refresh_required();
    dbg!(refresh_required);
}

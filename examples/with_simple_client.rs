use bubblehearth::client::BubbleHearthClient;
use bubblehearth::regionality::AccountRegion;

#[tokio::main]
async fn main() {
    let client = BubbleHearthClient::new(AccountRegion::US);
}

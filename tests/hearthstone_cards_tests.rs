mod hearthstone_cards_tests {
    use bubblehearth::client::BubbleHearthClient;
    use bubblehearth::localization::Locale;
    use bubblehearth::regionality::AccountRegion;

    pub fn get_default_client() -> BubbleHearthClient {
        get_regional_client(AccountRegion::US, Locale::EnglishUS)
    }

    pub fn get_regional_client(region: AccountRegion, locale: Locale) -> BubbleHearthClient {
        dotenvy::dotenv().expect("test client credentials unable to load");
        let client_id = std::env::var("CLIENT_ID").expect("test client ID not found");
        let client_secret = std::env::var("CLIENT_SECRET").expect("test client secret not found");
        BubbleHearthClient::new(client_id, client_secret, region, locale)
    }

    #[tokio::test]
    async fn returns_cards_when_no_query_provided() {
        // Arrange
        let client = get_default_client();

        // Act
        let cards_result = client.hearthstone().search_cards(None).await;
        let cards_ok = cards_result.is_ok();
        let cards = cards_result.unwrap();

        // Assert
        assert!(cards_ok);
        assert!(!cards.cards.is_empty());
        // cards.regions.into_iter().for_each(|r| {
        //     assert!(r.tag.is_none());
        //     assert!(r.name.is_none());
        //     assert!(r.id.is_none());
        // })
    }
}

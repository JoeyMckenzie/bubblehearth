mod classic_realm_tests {
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
    async fn returns_regions_index() {
        // arrange
        let client = get_default_client();

        // act
        let regions_index_result = client.classic.get_regions().await;
        let regions_index_ok = regions_index_result.is_ok();
        let regions_result = regions_index_result.unwrap();

        // assert
        assert!(regions_index_ok);
        assert!(!regions_result.regions.is_empty());
        regions_result.regions.into_iter().for_each(|r| {
            assert!(r.tag.is_none());
            assert!(r.name.is_none());
            assert!(r.id.is_none());
        })
    }

    #[tokio::test]
    async fn returns_region_with_valid_id() {
        // arrange
        let client = get_default_client();

        // act
        let region_result = client.classic.get_region(41).await;
        let region_result_ok = region_result.is_ok();
        let us_region = region_result.unwrap().unwrap();

        // assert
        assert!(region_result_ok);
        assert!(us_region.name.is_some());
        assert!(us_region.id.is_some());
        assert!(us_region.tag.is_some());
    }

    #[tokio::test]
    async fn returns_no_region_with_invalid_id() {
        // arrange
        let client = get_default_client();

        // act
        let region_result = client.classic.get_region(420).await;
        let region_result_ok = region_result.is_ok();
        let no_region = region_result.unwrap();

        // assert
        assert!(region_result_ok);
        assert!(no_region.is_none());
    }
}

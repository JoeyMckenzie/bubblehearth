mod classic_realm_tests {
    use bubblehearth::client::BubbleHearthClient;
    use bubblehearth::localization::{Locale, StringOrStructLocale};
    use bubblehearth::regionality::AccountRegion;
    use bubblehearth::timezone::Timezone;

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
    async fn returns_realms_index_with_given_locale() {
        // Arrange
        let client = get_regional_client(AccountRegion::KR, Locale::Korean);

        // Act
        let realms_index = client.classic().get_realms().await;
        let realms_index = realms_index.unwrap();
        let shimmering_flats = realms_index.realms.get(1).unwrap();

        // Assert
        assert_eq!(
            shimmering_flats.name,
            StringOrStructLocale::StringLocale("소금 평원".to_string())
        );
    }

    #[tokio::test]
    async fn returns_realms_index() {
        // Arrange
        let client = get_default_client();

        // Act
        let realms_index = client.classic().get_realms().await;

        // Assert
        assert!(realms_index.is_ok());
        assert!(!realms_index.unwrap().realms.is_empty());
    }

    #[tokio::test]
    async fn returns_realm_when_found() {
        // Arrange
        let client = get_default_client();

        // Act
        let realm = client.classic().get_realm("atiesh").await;
        let realm_ok = realm.is_ok();

        // Assert
        assert!(realm_ok);
        assert!(realm.unwrap().is_some());
    }

    #[tokio::test]
    async fn returns_no_realm_when_slug_invalid() {
        // Arrange
        let client = get_default_client();

        // Act
        let realm = client.classic().get_realm("not atiesh").await;
        let realm_ok = realm.is_ok();

        // Assert
        assert!(realm_ok);
        assert!(realm.unwrap().is_none());
    }

    #[tokio::test]
    async fn returns_all_realms_from_search_without_optional_params() {
        // Arrange
        let client = get_default_client();

        // Act
        let realm_search = client.classic().search_realms(None, None, None).await;
        let is_ok = realm_search.is_ok();
        let realm_data = realm_search.unwrap();

        // Assert
        assert!(is_ok);
        assert_eq!(realm_data.page, 1);
        assert_eq!(realm_data.page_size, 27);
        assert_eq!(realm_data.max_page_size, 100);
        assert_eq!(realm_data.page_count, 1);
        assert!(!realm_data.results.is_empty());
        assert_eq!(realm_data.results.len(), 27);
    }

    #[tokio::test]
    async fn returns_no_result_past_page_count() {
        // Arrange
        let client = get_default_client();

        // Act
        let realm_search = client.classic().search_realms(None, None, Some(12)).await;
        let is_ok = realm_search.is_ok();
        let realm_data = realm_search.unwrap();

        // Assert
        assert!(is_ok);
        assert_eq!(realm_data.page, 12);
        assert_eq!(realm_data.page_size, 0);
        assert_eq!(realm_data.max_page_size, 100);
        assert_eq!(realm_data.page_count, 1);
        assert!(realm_data.results.is_empty());
        assert_eq!(realm_data.results.len(), 0);
    }

    #[tokio::test]
    async fn returns_realms_from_timezone_when_specified() {
        // Arrange
        let client = get_default_client();

        // Act
        let classic = client.classic();
        let (us_west_realms, us_east_realms) = tokio::join!(
            classic.search_realms(Some(Timezone::AmericaLosAngeles), None, None),
            classic.search_realms(Some(Timezone::AmericaNewYork), None, None)
        );
        let us_west_realms_ok = us_west_realms.is_ok();
        let us_east_realms_ok = us_west_realms.is_ok();
        let us_west_realms_result = us_west_realms.unwrap();
        let us_east_realms_result = us_east_realms.unwrap();

        // Assert
        assert!(us_west_realms_ok);
        assert!(us_east_realms_ok);
        assert!(!us_west_realms_result.results.is_empty());
        assert!(!us_east_realms_result.results.is_empty());
        us_west_realms_result
            .results
            .into_iter()
            .for_each(|r| assert_eq!(r.data.timezone.unwrap(), Timezone::AmericaLosAngeles));
        us_east_realms_result
            .results
            .into_iter()
            .for_each(|r| assert_eq!(r.data.timezone.unwrap(), Timezone::AmericaNewYork));
    }
}

use planchet::{
    AddCollectedItemParams, Category, ClientBuilder, EditCollectedItemParams, Error,
    GetCollectedItemsParams, KnownApiError, OAuthTokenParams, SearchByImageRequest,
    SearchTypesParams, models::{self, GrantType, Orientation},
};
use futures::StreamExt;
use rust_decimal::Decimal;
use serde_json;

#[tokio::test]
async fn get_publication_full_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/publications/L106610")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
          "id": "L106610",
          "url": "https://numista.com/L106610",
          "type": "volume",
          "title": "Cast Chinese Coins",
          "bibliographical_notice": "David Hartill; 2017. <em>Cast Chinese Coins</em> (2<sup>nd</sup> Edition). Self-published, London, United Kingdom.",
          "edition": "2nd Edition",
          "languages": [
            "en"
          ],
          "year": "2017",
          "page_count": 453,
          "cover": "softcover",
          "isbn10": "1787194949",
          "isbn13": "9781787194946",
          "oclc_number": "1000342699",
          "contributors": [
            {
              "role": "author",
              "name": "David Hartill",
              "id": "369"
            }
          ],
          "publishers": [
            {
              "name": "Self-published",
              "id": "93"
            }
          ],
          "publication_places": [
            {
              "name": "London, United Kingdom",
              "geonames_id": "2643743"
            }
          ],
          "part_of": [
            {
              "type": "volume_group",
              "id": "L111322",
              "title": "Cast Chinese Coins"
            }
          ]
        }"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_publication("L106610").await.unwrap();

    mock.assert();
    assert_eq!(response.id, "L106610");
}

#[tokio::test]
async fn get_type_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/types/420")
      .match_query(mockito::Matcher::UrlEncoded("lang".into(), "de".into()))
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(r#"{
          "id": 420,
          "url": "https://en.numista.com/catalogue/pieces420.html",
          "title": "5 Cents - Victoria",
          "category": "coin",
          "issuer": {
            "code": "canada",
            "name": "Canada"
          },
          "min_year": 1858,
          "max_year": 1901,
          "type": "Standard circulation coin",
          "demonetization": {
              "is_demonetized": false
          },
          "tags": []
        }"#)
      .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .lang_code("de")
        .build()
        .unwrap();

    let response = client.get_type(420).await.unwrap();

    mock.assert();
    assert_eq!(response.id, 420);
    assert_eq!(response.title, "5 Cents - Victoria");
    assert_eq!(
        response.type_name.unwrap(),
        "Standard circulation coin"
    );
}

#[tokio::test]
async fn get_type_full_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/types/99700")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":99700,"url":"https:\/\/en.numista.com\/99700","title":"\u00bc Dollar \"Washington Quarter\" (George Rogers Clark National Historical Park, Indiana)","category":"coin","issuer":{"code":"etats-unis","name":"United States"},"min_year":2017,"max_year":2017,"type":"Circulating commemorative coins","ruler":[{"id":4720,"name":"Federal republic","wikidata_id":"Q30"}],"value":{"text":"\u00bc Dollar ","numeric_value":0.25,"numerator":1,"denominator":4,"currency":{"id":59,"name":"Dollar","full_name":"Dollar (1785-date)"}},"demonetization":{"is_demonetized":false},"size":24.3,"thickness":1.75,"shape":"Round","composition":{"text":"Copper-nickel clad copper"},"technique":{"text":"Milled"},"obverse":{"engravers":["William Cousins"],"designers":["John Flanagan"],"description":"The portrait in left profile of George Washington, the first President of the United States from 1789 to 1797, is accompanied with the motto \"IN GOD WE TRUST\" and the lettering \"LIBERTY\" surrounded with the denomination and the inscription \"UNITED STATES OF AMERICA\"","lettering":"UNITED STATES OF AMERICA\r\nIN \r\nGOD WE \r\nTRUST\r\nLIBERTY  P\r\nJF  WC\r\nQUARTER DOLLAR","lettering_scripts":[{"name":"Latin"}],"picture":"https:\/\/en.numista.com\/catalogue\/photos\/etats-unis\/5044-original.jpg","thumbnail":"https:\/\/en.numista.com\/catalogue\/photos\/etats-unis\/5044-180.jpg","picture_copyright":"Image courtesy of United States Mint"},"reverse":{"engravers":["Frank Morris","Michael Gaudioso"],"description":"George Rogers Clark leading his men through the flooded plains approaching Fort Sackville (frontier settlement of Vincennes).","lettering":"GEORGE ROGERS CLARK\r\nMG\r\nFM\r\nINDIANA   2017   E PLURIBUS UNUM","lettering_scripts":[{"name":"Latin"}],"picture":"https:\/\/en.numista.com\/catalogue\/photos\/etats-unis\/5045-original.jpg","thumbnail":"https:\/\/en.numista.com\/catalogue\/photos\/etats-unis\/5045-180.jpg","picture_copyright":"United States Mint","picture_copyright_url":"http:\/\/www.usmint.gov"},"series":"United States Mint's \"America the Beautiful\" Quarters Program","commemorated_topic":"George Rogers Clark National Historical Park, Indiana","tags":["Firearms","War","Park"],"references":[{"catalogue":{"id":3,"code":"KM"},"number":"657"}],"weight":5.67,"orientation":"coin","edge":{"description":"Reeded","picture":"https:\/\/en.numista.com\/catalogue\/photos\/etats-unis\/4024-original.jpg","thumbnail":"https:\/\/en.numista.com\/catalogue\/photos\/etats-unis\/4024-180.jpg","picture_copyright":"Cyrillius"},"mints":[{"id":"10","name":"United States Mint of Denver"},{"id":"11","name":"United States Mint of Philadelphia"},{"id":"12","name":"United States Mint of San Francisco"}]}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_type(99700).await.unwrap();

    mock.assert();
    assert_eq!(response.id, 99700);
    assert_eq!(response.url.unwrap().as_str(), "https://en.numista.com/99700");
    assert_eq!(response.title, "¼ Dollar \"Washington Quarter\" (George Rogers Clark National Historical Park, Indiana)");
    assert_eq!(response.category.to_string(), "Coin");
    let issuer = response.issuer.unwrap();
    assert_eq!(issuer.code, "etats-unis");
    assert_eq!(issuer.name, "United States");
    assert_eq!(response.min_year.unwrap(), 2017);
    assert_eq!(response.max_year.unwrap(), 2017);
    assert_eq!(response.type_name.unwrap(), "Circulating commemorative coins");
    let ruler = response.ruler.unwrap();
    assert_eq!(ruler.len(), 1);
    assert_eq!(ruler[0].id, 4720);
    assert_eq!(ruler[0].name, "Federal republic");
    assert_eq!(ruler[0].wikidata_id.as_ref().unwrap(), "Q30");
    let value = response.value.unwrap();
    assert_eq!(value.text.unwrap(), "¼ Dollar ");
    assert_eq!(value.numeric_value.unwrap(), Decimal::new(25, 2));
    assert_eq!(value.numerator.unwrap(), 1);
    assert_eq!(value.denominator.unwrap(), 4);
    let currency = value.currency.unwrap();
    assert_eq!(currency.id, 59);
    assert_eq!(currency.name, "Dollar");
    assert_eq!(currency.full_name, "Dollar (1785-date)");
    assert_eq!(response.demonetization.unwrap().is_demonetized, false);
    assert_eq!(response.size.unwrap(), Decimal::new(243, 1));
    assert_eq!(response.thickness.unwrap(), Decimal::new(175, 2));
    assert_eq!(response.shape.unwrap(), "Round");
    assert_eq!(response.composition.unwrap().text.unwrap(), "Copper-nickel clad copper");
    assert_eq!(response.technique.unwrap().text.unwrap(), "Milled");
    let obverse = response.obverse.unwrap();
    assert_eq!(obverse.engravers.unwrap(), vec!["William Cousins"]);
    assert_eq!(obverse.designers.unwrap(), vec!["John Flanagan"]);
    assert_eq!(obverse.description.unwrap(), "The portrait in left profile of George Washington, the first President of the United States from 1789 to 1797, is accompanied with the motto \"IN GOD WE TRUST\" and the lettering \"LIBERTY\" surrounded with the denomination and the inscription \"UNITED STATES OF AMERICA\"");
    assert_eq!(obverse.lettering.unwrap(), "UNITED STATES OF AMERICA\r\nIN \r\nGOD WE \r\nTRUST\r\nLIBERTY  P\r\nJF  WC\r\nQUARTER DOLLAR");
    let obverse_lettering_scripts = obverse.lettering_scripts.unwrap();
    assert_eq!(obverse_lettering_scripts.len(), 1);
    assert_eq!(obverse_lettering_scripts[0].name, "Latin");
    assert_eq!(obverse.picture.unwrap().as_str(), "https://en.numista.com/catalogue/photos/etats-unis/5044-original.jpg");
    assert_eq!(obverse.thumbnail.unwrap().as_str(), "https://en.numista.com/catalogue/photos/etats-unis/5044-180.jpg");
    assert_eq!(obverse.picture_copyright.unwrap(), "Image courtesy of United States Mint");
    let reverse = response.reverse.unwrap();
    assert_eq!(reverse.engravers.unwrap(), vec!["Frank Morris", "Michael Gaudioso"]);
    assert_eq!(reverse.description.unwrap(), "George Rogers Clark leading his men through the flooded plains approaching Fort Sackville (frontier settlement of Vincennes).");
    assert_eq!(reverse.lettering.unwrap(), "GEORGE ROGERS CLARK\r\nMG\r\nFM\r\nINDIANA   2017   E PLURIBUS UNUM");
    let reverse_lettering_scripts = reverse.lettering_scripts.unwrap();
    assert_eq!(reverse_lettering_scripts.len(), 1);
    assert_eq!(reverse_lettering_scripts[0].name, "Latin");
    assert_eq!(reverse.picture.unwrap().as_str(), "https://en.numista.com/catalogue/photos/etats-unis/5045-original.jpg");
    assert_eq!(reverse.thumbnail.unwrap().as_str(), "https://en.numista.com/catalogue/photos/etats-unis/5045-180.jpg");
    assert_eq!(reverse.picture_copyright.unwrap(), "United States Mint");
    assert_eq!(reverse.picture_copyright_url.unwrap().as_str(), "http://www.usmint.gov/");
    assert_eq!(response.series.unwrap(), "United States Mint's \"America the Beautiful\" Quarters Program");
    assert_eq!(response.commemorated_topic.unwrap(), "George Rogers Clark National Historical Park, Indiana");
    assert_eq!(response.tags.unwrap(), vec!["Firearms", "War", "Park"]);
    let references = response.references.unwrap();
    assert_eq!(references.len(), 1);
    assert_eq!(references[0].catalogue.id, 3);
    assert_eq!(references[0].catalogue.code, "KM");
    assert_eq!(references[0].number, "657");
    assert_eq!(response.weight.unwrap(), Decimal::new(567, 2));
    assert_eq!(response.orientation.unwrap(), Orientation::Coin);
    let edge = response.edge.unwrap();
    assert_eq!(edge.description.unwrap(), "Reeded");
    assert_eq!(edge.picture.unwrap().as_str(), "https://en.numista.com/catalogue/photos/etats-unis/4024-original.jpg");
    assert_eq!(edge.thumbnail.unwrap().as_str(), "https://en.numista.com/catalogue/photos/etats-unis/4024-180.jpg");
    assert_eq!(edge.picture_copyright.unwrap(), "Cyrillius");
    let mints = response.mints.unwrap();
    assert_eq!(mints.len(), 3);
    assert_eq!(mints[0].id, 10);
    assert_eq!(mints[0].name, "United States Mint of Denver");
    assert_eq!(mints[1].id, 11);
    assert_eq!(mints[1].name, "United States Mint of Philadelphia");
    assert_eq!(mints[2].id, 12);
    assert_eq!(mints[2].name, "United States Mint of San Francisco");
}

#[tokio::test]
async fn search_types_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/types")
      .match_query(mockito::Matcher::AllOf(vec![
        mockito::Matcher::UrlEncoded("q".into(), "victoria".into()),
        mockito::Matcher::UrlEncoded("lang".into(), "es".into()),
        mockito::Matcher::UrlEncoded("category".into(), "coin".into()),
      ]))
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(r#"{
          "count": 1,
          "types": [
            {
              "id": 420,
              "title": "5 Cents - Victoria",
              "category": "coin",
              "issuer": {
                "code": "canada",
                "name": "Canada"
              },
              "min_year": 1858,
              "max_year": 1901
            }
          ]
        }"#)
      .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .lang_code("es")
        .build()
        .unwrap();

    let params = SearchTypesParams::new()
        .q("victoria")
        .category(Category::Coin);
    let response = client.search_types(&params).await.unwrap();

    mock.assert();
    assert_eq!(response.count, 1);
    assert_eq!(response.types.len(), 1);
    assert_eq!(response.types[0].id, 420);
}

#[test]
fn search_types_params_year_date_test() {
    let params = SearchTypesParams::new().year(2000);
    // Note: year field is private but getters are not available, however this test was testing internal state or implementation detail?
    // Wait, the struct fields are not public. The previous test was inside lib.rs so it could access private fields.
    // I should check if I can access them or if I should rely on public interface.
    // The previous test `assert_eq!(params.year.unwrap(), "2000");` accessed the field directly.
    // In integration test, I cannot access private fields.
    // Since I cannot verify the internal state easily without public getters, and I don't want to add getters just for tests if not needed...
    // But `SearchTypesParams` implements Serialize. I can serialize it and check the output.

    let json = serde_json::to_value(&params).unwrap();
    assert_eq!(json["year"], "2000");

    let params = SearchTypesParams::new().year_range(1990, 2005);
    let json = serde_json::to_value(&params).unwrap();
    assert_eq!(json["year"], "1990-2005");

    let params = SearchTypesParams::new().date(1999);
    let json = serde_json::to_value(&params).unwrap();
    assert_eq!(json["date"], "1999");

    let params = SearchTypesParams::new().date_range(1980, 1985);
    let json = serde_json::to_value(&params).unwrap();
    assert_eq!(json["date"], "1980-1985");
}

#[tokio::test]
async fn stream_all_types_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    server
        .mock("GET", "/types")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("q".into(), "victoria".into()),
            mockito::Matcher::UrlEncoded("page".into(), "1".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "count": 2,
            "types": [
                { "id": 1, "title": "Type 1", "category": "coin", "issuer": {"code": "a", "name": "A"}, "min_year": 1, "max_year": 2 }
            ]
        }"#,
        )
        .create();

    server
        .mock("GET", "/types")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("q".into(), "victoria".into()),
            mockito::Matcher::UrlEncoded("page".into(), "2".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "count": 2,
            "types": [
                { "id": 2, "title": "Type 2", "category": "coin", "issuer": {"code": "b", "name": "B"}, "min_year": 3, "max_year": 4 }
            ]
        }"#,
        )
        .create();

    server
        .mock("GET", "/types")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("q".into(), "victoria".into()),
            mockito::Matcher::UrlEncoded("page".into(), "3".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "count": 2,
            "types": []
        }"#,
        )
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key")
        .base_url(url)
        .build()
        .unwrap();

    let params = SearchTypesParams::new().q("victoria");
    let stream = client.stream_all_types(params);

    let results: Vec<Result<models::SearchTypeResult, Error>> = stream.collect().await;
    let results: Result<Vec<models::SearchTypeResult>, Error> = results.into_iter().collect();
    let results = results.unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].id, 1);
    assert_eq!(results[1].id, 2);
}

#[tokio::test]
async fn get_issues_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/types/420/issues")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"id": 1, "is_dated": true}]"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key")
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_issues(420).await.unwrap();

    mock.assert();
    assert_eq!(response.len(), 1);
    assert_eq!(response[0].id, 1);
}

#[tokio::test]
async fn get_prices_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/types/420/issues/123/prices")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"currency": "USD", "prices": []}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_prices(420, 123, None).await.unwrap();

    mock.assert();
    assert_eq!(response.currency, iso_currency::Currency::USD);
}

#[tokio::test]
async fn get_issuers_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/issuers")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"count": 1, "issuers": [{"code": "canada", "name": "Canada"}]}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_issuers().await.unwrap();

    mock.assert();
    assert_eq!(response.count, 1);
    assert_eq!(response.issuers.len(), 1);
    assert_eq!(response.issuers[0].code, "canada");
}

#[tokio::test]
async fn get_mints_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/mints")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"count": 1, "mints": [{"id": 1}]}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_mints().await.unwrap();

    mock.assert();
    assert_eq!(response.count, 1);
    assert_eq!(response.mints.len(), 1);
    assert_eq!(response.mints[0].id, 1);
}

#[tokio::test]
async fn get_mint_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/mints/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": "1"}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_mint(1).await.unwrap();

    mock.assert();
    assert_eq!(response.id, 1);
}

#[tokio::test]
async fn get_catalogues_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/catalogues")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"count": 1, "catalogues": [{"id": 1, "code": "KM", "title": "Test", "author": "Test", "publisher": "Test"}]}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_catalogues().await.unwrap();

    mock.assert();
    assert_eq!(response.count, 1);
    assert_eq!(response.catalogues.len(), 1);
    assert_eq!(response.catalogues[0].id, 1);
}

#[tokio::test]
async fn get_publication_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/publications/L106610")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": "L106610", "url": "https://example.com", "type": "volume", "title": "Test", "languages": []}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_publication("L106610").await.unwrap();

    mock.assert();
    assert_eq!(response.id, "L106610");
}

#[tokio::test]
async fn get_user_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/users/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"username": "test"}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_user(1).await.unwrap();

    mock.assert();
    assert_eq!(response.username, "test");
}

#[tokio::test]
async fn get_user_collections_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/users/1/collections")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"count": 1, "collections": [{"id": 1, "name": "Test"}]}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_user_collections(1).await.unwrap();

    mock.assert();
    assert_eq!(response.count, 1);
    assert_eq!(response.collections.len(), 1);
    assert_eq!(response.collections[0].id, 1);
}

#[tokio::test]
async fn get_collected_items_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/users/1/collected_items")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"item_count": 1, "item_for_swap_count": 0, "item_type_count": 1, "item_type_for_swap_count": 0, "items": [{"id": 1, "quantity": 1, "type": {"id": 1, "title": "Test", "category": "coin"}, "for_swap": false}]}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let params = GetCollectedItemsParams::new();
    let response = client.get_collected_items(1, &params).await.unwrap();

    mock.assert();
    assert_eq!(response.item_count, 1);
    assert_eq!(response.items.len(), 1);
    assert_eq!(response.items[0].id, 1);
}

#[tokio::test]
async fn add_collected_item_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("POST", "/users/1/collected_items")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": 1, "quantity": 1, "type": {"id": 1, "title": "Test", "category": "coin"}, "for_swap": false}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let item = AddCollectedItemParams {
        type_id: 1,
        issue: None,
        quantity: None,
        grade: None,
        for_swap: None,
        private_comment: None,
        public_comment: None,
        price: None,
        collection: None,
        storage_location: None,
        acquisition_place: None,
        acquisition_date: None,
        serial_number: None,
        internal_id: None,
        weight: None,
        size: None,
        axis: None,
        grading_details: None,
    };
    let response = client.add_collected_item(1, &item).await.unwrap();

    mock.assert();
    assert_eq!(response.id, 1);
}

#[tokio::test]
async fn get_collected_item_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/users/1/collected_items/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": 1, "quantity": 1, "type": {"id": 1, "title": "Test", "category": "coin"}, "for_swap": false}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_collected_item(1, 1).await.unwrap();

    mock.assert();
    assert_eq!(response.id, 1);
}

#[tokio::test]
async fn edit_collected_item_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("PATCH", "/users/1/collected_items/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": 1, "quantity": 1, "type": {"id": 1, "title": "Test", "category": "coin"}, "for_swap": false}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let item = EditCollectedItemParams {
        type_id: None,
        issue: None,
        quantity: None,
        grade: None,
        for_swap: None,
        private_comment: None,
        public_comment: None,
        price: None,
        collection: None,
        storage_location: None,
        acquisition_place: None,
        acquisition_date: None,
        serial_number: None,
        internal_id: None,
        weight: None,
        size: None,
        axis: None,
        grading_details: None,
    };
    let response = client.edit_collected_item(1, 1, &item).await.unwrap();

    mock.assert();
    assert_eq!(response.id, 1);
}

#[tokio::test]
async fn delete_collected_item_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("DELETE", "/users/1/collected_items/1")
        .with_status(204)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.delete_collected_item(1, 1).await;

    mock.assert();
    assert!(response.is_ok());
}

#[tokio::test]
async fn get_oauth_token_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("GET", "/oauth_token")
        .match_query(mockito::Matcher::UrlEncoded("grant_type".into(), "client_credentials".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"access_token": "test", "token_type": "bearer", "expires_in": 3600, "user_id": 1}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let params = OAuthTokenParams {
        grant_type: GrantType::ClientCredentials,
        code: None,
        client_id: None,
        client_secret: None,
        redirect_uri: None,
        scope: None,
    };
    let response = client.get_oauth_token(&params).await.unwrap();

    mock.assert();
    assert_eq!(response.access_token, "test");
}

#[tokio::test]
async fn search_by_image_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server.mock("POST", "/search_by_image")
        .match_body(mockito::Matcher::Json(serde_json::json!({
            "category": null,
            "images": [
                {
                    "mime_type": "image/jpeg",
                    "image_data": "jpeg_data"
                },
                {
                    "mime_type": "image/png",
                    "image_data": "png_data"
                }
            ],
            "max_results": null
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"count": 0, "types": []}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let request = SearchByImageRequest {
        category: None,
        images: vec![
            models::request::Image {
                mime_type: models::request::MimeType::Jpeg,
                image_data: "jpeg_data".to_string(),
            },
            models::request::Image {
                mime_type: models::request::MimeType::Png,
                image_data: "png_data".to_string(),
            },
        ],
        max_results: None,
    };
    client.search_by_image(&request).await.unwrap();

    mock.assert();
}

#[tokio::test]
async fn unauthorized_error_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server
        .mock("GET", "/types/420")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error_message": "Invalid API key"}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_type(420).await;

    mock.assert();
    assert!(response.is_err());
    match response.err().unwrap() {
        Error::ApiError(e) => {
            assert_eq!(e.status, 401);
            assert_eq!(e.kind, Some(KnownApiError::Unauthorized));
        }
        _ => panic!("Expected ApiError"),
    }
}

#[tokio::test]
async fn not_found_error_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server
        .mock("GET", "/types/999999")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error_message": "Not found"}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key")
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_type(999999).await;

    mock.assert();
    assert!(response.is_err());
    match response.err().unwrap() {
        Error::ApiError(e) => {
            assert_eq!(e.status, 404);
            assert_eq!(e.kind, Some(KnownApiError::NotFound));
        }
        _ => panic!("Expected ApiError"),
    }
}

#[tokio::test]
async fn invalid_parameter_error_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server
        .mock("GET", "/types")
        .match_query(mockito::Matcher::UrlEncoded("q".into(), "a".repeat(101)))
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error_message": "Invalid parameter"}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key")
        .base_url(url)
        .build()
        .unwrap();

    let params = SearchTypesParams::new().q("a".repeat(101));
    let response = client.search_types(&params).await;

    mock.assert();
    assert!(response.is_err());
    match response.err().unwrap() {
        Error::ApiError(e) => {
            assert_eq!(e.status, 400);
            assert_eq!(e.message, "Invalid parameter");
            assert_eq!(e.kind, Some(KnownApiError::InvalidParameter));
        }
        _ => panic!("Expected ApiError"),
    }
}

#[tokio::test]
async fn rate_limit_exceeded_error_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server
        .mock("GET", "/types/123")
        .with_status(429)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error_message": "Rate limit exceeded"}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key")
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_type(123).await;

    mock.assert();
    assert!(response.is_err());
    match response.err().unwrap() {
        Error::ApiError(e) => {
            assert_eq!(e.status, 429);
            assert_eq!(e.kind, Some(KnownApiError::RateLimitExceeded));
        }
        _ => panic!("Expected ApiError"),
    }
}

#[tokio::test]
async fn no_user_associated_error_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server
        .mock("GET", "/oauth_token")
        .match_query(mockito::Matcher::UrlEncoded(
            "grant_type".into(),
            "client_credentials".into(),
        ))
        .with_status(501)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error_message": "No user associated"}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key")
        .base_url(url)
        .build()
        .unwrap();

    let params = OAuthTokenParams {
        grant_type: GrantType::ClientCredentials,
        code: None,
        client_id: None,
        client_secret: None,
        redirect_uri: None,
        scope: None,
    };
    let response = client.get_oauth_token(&params).await;

    mock.assert();
    assert!(response.is_err());
    match response.err().unwrap() {
        Error::ApiError(e) => {
            assert_eq!(e.status, 501);
            assert_eq!(e.kind, Some(KnownApiError::NoUserAssociatedWithApiKey));
        }
        _ => panic!("Expected ApiError"),
    }
}

#[tokio::test]
async fn generic_api_error_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();

    let mock = server
        .mock("GET", "/types/420")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error_message": "Internal Server Error"}"#)
        .create();

    let client = ClientBuilder::new()
        .api_key("test_key".to_string())
        .base_url(url)
        .build()
        .unwrap();

    let response = client.get_type(420).await;

    mock.assert();
    assert!(response.is_err());
    match response.err().unwrap() {
        Error::ApiError(e) => {
            assert_eq!(e.message, "Internal Server Error");
            assert_eq!(e.status, 500);
            assert!(e.kind.is_none());
        }
        _ => panic!("Expected a generic ApiError"),
    }
}

use crate::test_utils::start_test_server;

use wiremock::{Mock, ResponseTemplate, matchers::{method, path}};

#[actix_web::test]
async fn test_get_ping() {
    let app = start_test_server().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/ping", &app.address))
        .header("Content-Type", "text/plain")
        .send()
        .await
        .expect("failed to execute GET /ping request");

    Mock::given(path("/third_party_api"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.test_server)
        .await;

    assert!(response.status().is_success())
}
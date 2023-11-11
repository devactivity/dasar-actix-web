use crate::test_utils::start_test_server;

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

    assert!(response.status().is_success())
}
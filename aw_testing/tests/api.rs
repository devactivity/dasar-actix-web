use std::net::TcpListener;
use aw_testing::server::start;

pub struct MyApp {
    pub address: String
}

async fn start_server() -> MyApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server = start(listener)
        .expect("Failed to bind address");

    let _ = actix_web::rt::spawn(server);

    MyApp { address }
}

#[actix_web::test]
async fn test_index_get() {
    let app = start_server().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/", &app.address))
        .header("Content-Type", "text/plain")
        .send()
        .await
        .expect("Failed to execute GET / request");

    assert!(response.status().is_success())
}
use crate::test_utils::start_test_server;

#[actix_web::test]
async fn get_article_list_returns_a_200_if_it_success() {
    // Arrange
    let app = start_test_server().await;

    // Act
    let response = app.payload_for_get("api/v1/articles").await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn create_article_returns_a_201_for_valid_payload_data() {
    // Arrange
    let app = start_test_server().await;

    // Create a JSON payload as a serde_json::Value
    let payload = serde_json::json!({
        "user": {
            "username": "test_devactivity",
            "email": "test@devactivity.com",
            "password": "12345678"
        }
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_post(body_data.into(), "api/v1/users/register").await;

    // Assert
    assert_eq!(201, response.status().as_u16());

    // Create a JSON payload as a serde_json::Value
    let payload = serde_json::json!({
        "body": "this is body article",
        "description": "the most interesting topic",
        "tagList": [
          "interest"
        ],
        "title": "the-interesting-topic"
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_post(body_data.into(), format!("api/v1/articles/{}", "test_devactivity").as_str()).await;

    // Assert
    assert_eq!(201, response.status().as_u16());
}

#[actix_web::test]
async fn create_article_returns_a_400_for_invalid_payload_data() {
    // Arrange
    let app = start_test_server().await;

    // Create a JSON payload as a serde_json::Value
    let payload = serde_json::json!({
        "user": {
            "username": "test_devactivity",
            "email": "test@devactivity.com",
            "password": "12345678"
        }
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_post(body_data.into(), "api/v1/users/register").await;

    // Assert
    assert_eq!(201, response.status().as_u16());

    // Create a JSON payload as a serde_json::Value
    let payload = serde_json::json!({
        "body": "",
        "description": "",
        "tagList": [""],
        "title": ""
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_post(body_data.into(), format!("api/v1/articles/{}", "test_devactivity").as_str()).await;

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[actix_web::test]
async fn get_article_feed_returns_a_200_for_invalid_payload_data() {
    // Arrange
    let app = start_test_server().await;

    // Create a JSON payload as a serde_json::Value
    let payload = serde_json::json!({
        "user": {
            "username": "test_devactivity",
            "email": "test@devactivity.com",
            "password": "12345678"
        }
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_post(body_data.into(), "api/v1/users/register").await;

    // Assert
    assert_eq!(201, response.status().as_u16());

    // Act
    let response = app.payload_for_get(format!("api/v1/articles/feed/{}?limit=20&offset=0", "test_devactivity").as_str()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

// NOTES: the rest is yours
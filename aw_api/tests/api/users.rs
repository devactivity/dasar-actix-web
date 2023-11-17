use crate::test_utils::start_test_server;

#[actix_web::test]
async fn register_user_returns_a_201_for_valid_payload_data_and_login_should_be_success() {
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
        "user": {
            "email": "test@devactivity.com",
            "password": "12345678"
        }
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_post(body_data.into(), "api/v1/users/login").await;

    // Assert
    assert_eq!(201, response.status().as_u16());
}

#[actix_web::test]
async fn login_user_returns_a_500_if_user_does_not_exist() {
    // Arrange
    let app = start_test_server().await;

    // Create a JSON payload as a serde_json::Value
    let payload = serde_json::json!({
        "user": {
            "email": "unknown@unknown.com",
            "password": "abcd1234"
        }
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_post(body_data.into(), "api/v1/users/login").await;

    // Assert
    assert_eq!(500, response.status().as_u16());
}

#[actix_web::test]
async fn register_user_returns_a_400_for_invalid_payload_data() {
    // Arrange
    let app = start_test_server().await;

    // Create a JSON payload as a serde_json::Value
    let payload = serde_json::json!({
        "user": {
            "username": "$!#$",             // <= only for alphanumeric/underscore characters
            "email": "testexample.com",     // <= is not a valid email address
            "password": "1234"              // <= must be 8-72 characters long
        }
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_post(body_data.into(), "api/v1/users/register").await;

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[actix_web::test]
async fn update_user_returns_a_201_for_valid_payload_data() {
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
        "user": {
            "username": "test_devactivity",
            "email": "new@devactivity.com",
            "password": "12345678",
            "bio": "i am a human"
        }
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_put(body_data.into(), "api/v1/users/update").await;

    // Assert
    assert_eq!(201, response.status().as_u16());
}

#[actix_web::test]
async fn update_user_returns_a_404_for_not_found_data() {
    // Arrange
    let app = start_test_server().await;

    // Create a JSON payload as a serde_json::Value
    let payload = serde_json::json!({
        "user": {
            "username": "test_devactivity",
            "email": "new@devactivity.com",
            "password": "12345678",
            "bio": "i am a human"
        }
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_put(body_data.into(), "api/v1/users/update").await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[actix_web::test]
async fn update_user_returns_a_400_for_invalid_payload_data() {
    // Arrange
    let app = start_test_server().await;

    // Create a JSON payload as a serde_json::Value
    let payload = serde_json::json!({
        "user": {
            "username": "$!#$",             // <= only for alphanumeric/underscore characters
            "email": "testexample.com",     // <= is not a valid email address
            "password": "1234"              // <= must be 8-72 characters long
        }
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_put(body_data.into(), "api/v1/users/update").await;

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[actix_web::test]
async fn delete_user_returns_a_200_for_success() {
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
        "user": {
            "username": "test_devactivity"
        }
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_delete(body_data.into(), "api/v1/users/delete").await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn delete_user_returns_a_404_for_not_found_data() {
    // Arrange
    let app = start_test_server().await;

    // Create a JSON payload as a serde_json::Value
    let payload = serde_json::json!({
        "user": {
            "username": "test_devactivity"
        }
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();

    // Act
    let response = app.payload_for_delete(body_data.into(), "api/v1/users/delete").await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}


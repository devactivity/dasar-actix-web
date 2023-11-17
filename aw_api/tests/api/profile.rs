use crate::test_utils::start_test_server;

#[actix_web::test]
async fn get_profile_returns_a_200_if_it_success() {
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

    let payload = "test_devactivity";
    // Act
    let response = app.payload_for_get(format!("api/v1/profiles/{}", payload).as_str()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn get_profile_returns_a_404_for_not_found_data() {
    // Arrange
    let app = start_test_server().await;

     let payload = "unknown";
    // Act
    let response = app.payload_for_get(format!("api/v1/profiles/{}", payload).as_str()).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[actix_web::test]
async fn follow_a_profile_returns_a_201_if_it_success_and_unfollow_it_returns_a_202() {
    // Arrange
    let app = start_test_server().await;

    for index in 0..2 {
        let payload = serde_json::json!({
            "user": {
                "username": format!("test_devactivity{}", index),
                "email": format!("test{}@devactivity.com", index),
                "password": "12345678"
            }
        });

        // Serialize the JSON payload into a string
        let body_data = serde_json::to_string(&payload).unwrap();

        // Act
        let response = app.payload_for_post(body_data.into(), "api/v1/users/register").await;

        // Assert
        assert_eq!(201, response.status().as_u16());
    }

    let payload = serde_json::json!({
        "email": "test1@devactivity.com",
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();
    // Act
    let response = app.payload_for_post(body_data.into(), format!("api/v1/profiles/{}/follow", "test_devactivity0").as_str()).await;

    // Assert
    assert_eq!(201, response.status().as_u16());

    let body_data = serde_json::to_string(&payload).unwrap();
    // Act
    let response = app.payload_for_delete(body_data.into(), format!("api/v1/profiles/{}/follow", "test_devactivity1").as_str()).await;

    // Assert
    assert_eq!(202, response.status().as_u16());
}

#[actix_web::test]
async fn follow_a_profile_returns_a_500_if_user_does_not_exist() {
    // Arrange
    let app = start_test_server().await;

    let payload = serde_json::json!({
        "email": "test1@devactivity.com",
    });

    // Serialize the JSON payload into a string
    let body_data = serde_json::to_string(&payload).unwrap();
    // Act
    let response = app.payload_for_post(body_data.into(), format!("api/v1/profiles/{}/follow", "test_devactivity").as_str()).await;

    // Assert
    assert_eq!(500, response.status().as_u16());
}

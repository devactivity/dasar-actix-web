use crate::test_utils::start_test_server;

#[actix_web::test]
async fn get_tags_returns_a_200_if_it_success() {
    // Arrange
    let app = start_test_server().await;

    // Act
    let response = app.payload_for_get("api/v1/tags").await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

use actix_web::{web, HttpResponse};
use sqlx::{self, PgPool};
use uuid::Uuid;
use validator::Validate;

use crate::schemas::*;
use crate::errors::Error as AppError;
use crate::utils::validation_errors_response;

/// Return profile of a User
#[utoipa::path(
    get,
    path = "/api/v1/profiles/{username}",
    tag = "profiles",
    responses(
        (status = 200, description = "Success", body = ProfileResponse),
        (status = 400, description = "Bad request")
    ),
    params(
        ("username" = String, Path, description = "Username of a user"),
    )
)]
pub async fn get_profile(
    (form, pool): (web::Path<Profile>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    let user_profile = form.into_inner();

    // Validate the user input
    // user_profile.validate()?;

    // Validate the user input
    let validation_result = user_profile.validate();
    if let Err(validation_errors) = validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

    // Access the PgPool from the Data container
    let pool = pool.get_ref();

    // Create a query to fetch the user data
    let user_data = sqlx::query_as::<_, (Uuid, String, Option<String>)>(
        "SELECT id, username, bio FROM users WHERE username = $1"
    )
    .bind(&user_profile.username)
    .fetch_optional(pool)
    .await?;

    if let Some((user_id, username, bio)) = user_data {
        // Check if the bio is null or has a value
        let bio_value = bio.unwrap_or_default();

        // Create a query to check if a record exists in the followers table
        let is_following = sqlx::query("SELECT 1 FROM followers WHERE user_id = $1 AND follower_id = $2")
            .bind(&user_id)
            .bind(&user_id)
            .fetch_optional(pool)
            .await?
            .is_some();

        // Construct the JSON response
        let response = serde_json::json!({
            "profile": {
                // "id": user_id.to_string(),
                "username": username,
                "bio": bio_value,
                "is_following": is_following,
                // Add other fields here
            },
        });

        // Return a successful response
        Ok(HttpResponse::Ok().json(response))
    } else {
        // No record found
        let not_found_response = serde_json::json!({
            "message": "Record not found for the provided username",
        });
        Ok(HttpResponse::NotFound().json(not_found_response))
    }
}

/// Follow a User
#[utoipa::path(
    post,
    path = "/api/v1/profiles/{username}/follow",
    tag = "profiles",
    responses(
        (status = 201, description = "Success", body = ProfileResponse),
        (status = 400, description = "Bad request")
    ),
    params(
        ("username" = String, Path, description = "Username of a user you want to follow"),
    )
)]
pub async fn follow_profile(
    (form, pool): (web::Path<Profile>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    let user_info = form.into_inner();

    // Validate the user input
    let validation_result = user_info.validate();
    if let Err(validation_errors) = validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

    // Access the PgPool from the Data container
    let pool = pool.get_ref();

    // Fetch the current user
    let current_user= sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1 LIMIT 1",
        &user_info.username
    )
    .fetch_one(pool)
    .await
    .map_err(|_| {
        AppError::InternalServerError
    })?;

    // Fetch the target user data
    let target_user = sqlx::query_as::<_, (Uuid, String, Option<String>)>(
        "SELECT id, username, bio FROM users WHERE username = $1 LIMIT 1"
    )
    .bind(&user_info.username)
    .fetch_optional(pool)
    .await?;


    if let Some((following_user_id, following_username, following_user_bio)) = target_user {
        if &user_info.username == &following_username {
            return Err(AppError::UnprocessableEntity(
                serde_json::json!({"error": "You cannot follow yourself"}),
            ));
        }

        let follower_id = current_user.id;

        // Insert the follower record
        sqlx::query("INSERT INTO followers (user_id, follower_id) VALUES ($1, $2)")
            .bind(&following_user_id)
            .bind(&follower_id)
            .execute(pool)
            .await
            .map_err(|_| {
                AppError::InternalServerError
            })?;

        // Construct the JSON response
        let response = serde_json::json!({
            "profile": {
                "username": &following_username,
                "bio": following_user_bio,
                "is_following": true,
            },
        });

        // Return a successful response
        Ok(HttpResponse::Created().json(response))
    } else {
        // No record found
        let not_found_response = serde_json::json!({
            "message": "Record not found for the provided username",
        });
        Ok(HttpResponse::NotFound().json(not_found_response))
    }
}

/// Unfollow a User
#[utoipa::path(
    delete,
    path = "/api/v1/profiles/{username}/follow",
    tag = "profiles",
    responses(
        (status = 201, description = "Success"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("username" = String, Path, description = "Username of a user you followed"),
    )
)]
pub async fn unfollow_profile(
    (form, pool): (web::Path<Profile>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    let user_info = form.into_inner();

    // Validate the user input
    let validation_result = user_info.validate();
    if let Err(validation_errors) = validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

    // Access the PgPool from the Data container
    let pool = pool.get_ref();

    // Fetch the current user
    let current_user= sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1 LIMIT 1",
        &user_info.username
    )
    .fetch_one(pool)
    .await
    .map_err(|_| {
        AppError::NotFound(serde_json::json!({"error": "Record not found for the provided username"}))
    })?;

    // Fetch the target user data
    let target_user = sqlx::query_as::<_, (Uuid, String, Option<String>)>(
        "SELECT id, username, bio FROM users WHERE username = $1 LIMIT 1"
    )
    .bind(&user_info.username)
    .fetch_optional(pool)
    .await?;

    if let Some((following_user_id, following_username, following_user_bio)) = target_user {
        let follower_id = current_user.id;

        // Delete the follower record
        sqlx::query("DELETE FROM followers WHERE user_id = $1 AND follower_id = $2")
            .bind(&following_user_id)
            .bind(&follower_id)
            .execute(pool)
            .await
            .map_err(|_| {
                AppError::InternalServerError
            })?;

        // Construct the JSON response
        let response = serde_json::json!({
            "profile": {
                "username": &following_username,
                "bio": following_user_bio,
                "is_following": false,
            },
        });

        // Return a successful response
        Ok(HttpResponse::Accepted().json(response))
    } else {
        // No record found
        let not_found_response = serde_json::json!({
            "message": "Record not found for the provided username",
        });
        Ok(HttpResponse::NotFound().json(not_found_response))
    }
}

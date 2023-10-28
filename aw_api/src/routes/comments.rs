use actix_web::{web, HttpResponse, ResponseError, http::StatusCode};
use futures::future::join_all;
use sqlx::{self, PgPool, Postgres};
use uuid::Uuid;
use sqlx::Row;
use validator::Validate;

use crate::schemas::*;
use crate::errors::Error as AppError;
use crate::utils::validation_errors_response;

/// Return list of articles with your comment
#[utoipa::path(
    get,
    path = "/api/v1/articles/comments/{slug}",
    tag = "articles",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("slug" = String, Path, description = "an article slug"),
        ("username" = String, Query, description = "Username of a user")
    )
)]
pub async fn get_articles_comments(
    (params, username, pool): (web::Path<ArticlePath>, web::Query<UserForArticle>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    let pool = pool.get_ref();

    let user_info = username.into_inner();

    // Validate the user input
    let user_validation_result = user_info.validate();
    if let Err(validation_errors) = user_validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

    let user= sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1 LIMIT 1",
        &user_info.username
    )
    .fetch_one(pool)
    .await
    .map_err(|_| {
        AppError::InternalServerError
    })?;

    let article_id: Vec<Uuid> = sqlx::query_as::<_, (Uuid,)>("SELECT id FROM articles WHERE slug = $1")
        .bind(&params.slug)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|(id,)| id)
        .collect();

    let comments = sqlx::query_as!(
        Comment,
        "SELECT * FROM comments WHERE article_id = $1",
        article_id.get(0)
    )
    .fetch_all(pool)
    .await?;

    let comment_response = get_comment_list_response(comments, Some(user.id), pool).await?;

    // Return the article response as an HTTP response
    Ok(HttpResponse::Ok().json(comment_response))
}

/// Add a comment to an articles
#[utoipa::path(
    post,
    path = "/api/v1/articles/comments/{slug}",
    tag = "articles",
    responses(
        (status = 201, description = "Success"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("slug" = String, Path, description = "an article slug"),
        ("username" = String, Query, description = "Username of a user")
    ),
    request_body = AddComment
)]
pub async fn add_articles_comments(
    (form, path, username, pool): (web::Json<AddComment>,  web::Path<ArticlePath>, web::Query<UserForArticle>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    let comment_data: AddComment = form.into_inner();
    let user_info = username.into_inner();

    // Validate the user input
    let article_validation_result = comment_data.validate();
    if let Err(validation_errors) = article_validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

    // Validate the user input
    let user_validation_result = user_info.validate();
    if let Err(validation_errors) = user_validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

    // Access the PgPool from the Data container
    let pool = pool.get_ref();

    // Fetch the current user
    let user= sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1 LIMIT 1",
        &user_info.username
    )
    .fetch_one(pool)
    .await
    .map_err(|_| {
        AppError::InternalServerError
    })?;

    let article_id: Vec<Uuid> = sqlx::query_as::<_, (Uuid,)>("SELECT id FROM articles WHERE slug = $1")
        .bind(&path.slug)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|(id,)| id)
        .collect();

    let user_id = user.id;
    let article_id =  article_id.get(0).unwrap().to_owned();
    let new_comment = NewComment {
        article_id,
        user_id,
        body: comment_data.body,
    };

    let query = sqlx::query("INSERT INTO comments (article_id, user_id, body) VALUES ($1, $2, $3)")
        .bind(&new_comment.article_id)
        .bind(&new_comment.user_id)
        .bind(&new_comment.body);

    // Execute the query on the pool
    match query.execute(pool).await {
        Ok(_) => {
            // Now, perform a SELECT query to retrieve the inserted record
            let select_query = sqlx::query("SELECT * FROM comments WHERE article_id = $1 AND user_id = $2 AND body = $3")
                .bind(&new_comment.article_id)
                .bind(&new_comment.user_id)
                .bind(&new_comment.body);

            match select_query.fetch_one(pool).await {
                Ok(inserted_comment) => {
                    let comment_id = inserted_comment.get("id");
                    let comment_response = get_comment_response(comment_id, Some(user_id), pool).await?;

                    // Return the article response as an HTTP response
                    Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(comment_response))
                }
                Err(err) => {
                    let custom_err: AppError = err.into();
                    Ok(custom_err.error_response())
                }
            }
        }
        Err(err) => {
            let custom_err: AppError = err.into();
            Ok(custom_err.error_response())
        }
    }
}

/// Add a comment to an articles
#[utoipa::path(
    delete,
    path = "/api/v1/articles/comments/{slug}",
    tag = "articles",
    responses(
        (status = 201, description = "Success"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("slug" = String, Path, description = "an article slug"),
        ("comment_id" = i64, Path, description = "comment id"),
        ("username" = String, Query, description = "Username of a user")
    ),
    request_body = AddComment
)]
pub async fn delete_articles_comments(
    (form, username, pool): (web::Path<ArticleCommentPath>, web::Query<UserForArticle>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    let user_info = username.into_inner();

    // Validate the user input
    let user_validation_result = user_info.validate();
    if let Err(validation_errors) = user_validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

    // Access the PgPool from the Data container
    let pool = pool.get_ref();

    let (comment_id, user_id): (Uuid, Uuid) = sqlx::query_as::<_, (Uuid, Uuid)>("SELECT id, user_id FROM comments WHERE id = $1")
        .bind(&form.comment_id)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            AppError::InternalServerError
        })?;

    // Fetch the current user
    let user= sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1 LIMIT 1",
        &user_info.username
    )
    .fetch_one(pool)
    .await
    .map_err(|_| {
        AppError::InternalServerError
    })?;

    if user.id != user_id {
        return Err(AppError::Forbidden(serde_json::json!({
            "error": "user is not the author of article in question",
        })));
    }

    let query = sqlx::query("DELETE FROM comments WHERE id = $1")
        .bind(&comment_id);

    match query.execute(pool).await {
        Ok(res) => {
            if res.rows_affected() > 0 {
                let success_response = serde_json::json!({
                    "message": "Record deleted successfully",
                });
                Ok(HttpResponse::Ok().json(success_response))
            } else {
                // No rows were affected, indicating the id was not found.
                let not_found_response = serde_json::json!({
                    "message": "Record not found for the provided id",
                });
                Ok(HttpResponse::NotFound().json(not_found_response))
            }
        }
        Err(err) => {
            let custom_err: AppError = err.into();
            Ok(custom_err.error_response())
        }
    }
}

// Some helpers for this route ------------------------------------------------------------
async fn get_comment_list_response(
    comments: Vec<Comment>,
    user_id: Option<Uuid>,
    pool: &PgPool,
) -> Result<CommentListResponse, AppError> {
    let futures = comments
        .iter()
        .map(|comment| {
            let comment_id = comment.id.to_owned();

            async move {
                match get_comment_response(comment_id, user_id, pool).await {
                    Ok(response) => Ok(response.comment),
                    Err(e) => Err(e),
                }
            }
        });

    let results: Vec<Result<CommentResponseInner, AppError>> = join_all(futures).await;

    let comment_list: Result<Vec<CommentResponseInner>, AppError> = results
        .into_iter()
        .collect();

    match comment_list {
        Ok(comments) => {
            Ok(CommentListResponse {
                comments,
            })
        }
        Err(e) => Err(e),
    }
}

async fn get_comment_response(
    comment_id: i32,
    user_id: Option<Uuid>,
    pool: &PgPool,
) -> Result<CommentResponse, AppError> {
    // Query for comment
    let comment: Comment = sqlx::query_as!(
        Comment,
        r#"
        SELECT * FROM comments
        WHERE id = $1
        "#,
        comment_id
    )
    .fetch_one(pool)
    .await?;

    // Query for commenter
    let commenter: User = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users
        WHERE id = $1
        "#,
        comment.user_id
    )
    .fetch_one(pool)
    .await?;

    let following = match user_id {
        Some(user_id) => {
            let follower_exists: Option<i32> = sqlx::query_scalar::<Postgres, i32>(
                "SELECT 1 FROM followers
                WHERE user_id = $1 AND follower_id = $2"
            )
            .bind(commenter.id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

            follower_exists.is_some()
        },
        None => false,
    };

    Ok(CommentResponse {
        comment: CommentResponseInner {
            id: comment.id,
            created_at: CustomDateTime(comment.created_at),
            updated_at: CustomDateTime(comment.updated_at),
            body: comment.body,
            author: ProfileResponseInner {
                username: commenter.username,
                bio: commenter.bio,
                following,
            },
        },
    })
}
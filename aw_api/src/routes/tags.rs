use actix_web::{web, HttpResponse};
use sqlx::{self, PgPool};

use crate::schemas::*;
use crate::errors::Error as AppError;

/// Return list of available tags
#[utoipa::path(
    get,
    path = "/api/v1/tags",
    tag = "tags",
    responses(
        (status = 200, description = "Success", body = TagsResponse),
        (status = 400, description = "Bad request")
    )
)]
pub async fn get_tags(
    pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    // Access the PgPool from the Data container
    let pool = pool.get_ref();

    let result: Result<Vec<ArticleTag>, AppError> = sqlx::query_as!(
        ArticleTag,
        // "SELECT DISTINCT tag_name FROM article_tags"
        "SELECT DISTINCT ON (tag_name) * FROM article_tags"
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        let custom_err: AppError = err.into();
        custom_err // Return your custom error directly
    });

    let tags: Vec<ArticleTag> = result?;
    let tag_list: Vec<String> = tags.iter().map(|tag| tag.tag_name.clone()).collect();

    Ok(HttpResponse::Ok().json(TagsResponse { tags: tag_list }))
}


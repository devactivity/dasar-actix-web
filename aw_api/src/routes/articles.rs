use actix_web::{web, HttpResponse, ResponseError, http::StatusCode};
use futures::future::{join_all, FutureExt};
use futures::future::try_join_all;
use sqlx::postgres::PgRow;
use sqlx::{self, PgPool};
use sluggify::sluggify::sluggify;
use blob_uuid::to_blob;
use uuid::Uuid;
use sqlx::Row;
use sqlx::FromRow;
use validator::Validate;

use crate::schemas::*;
use crate::errors::Error as AppError;
use crate::utils::validation_errors_response;

/// Return article list
#[utoipa::path(
    get,
    path = "/api/v1/articles",
    tag = "articles",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad request")
    )
)]
pub async fn get_articles(
    (params, pool): (web::Query<ArticlesParams>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    // Access the PgPool from the Data container
    let pool = pool.get_ref();

    let mut query = format!("SELECT * FROM articles AS a");

    if let Some(ref author_name) = params.author {
        query.push_str(" INNER JOIN users AS u ON u.id = a.author_id");
        query.push_str(&format!(" WHERE u.username = '{}'", author_name));
    }

    if let Some(ref username_favorited_by) = params.favorited {
        query.push_str(" INNER JOIN favorite_articles AS fa ON fa.article_id = a.id");
        query.push_str(" INNER JOIN users AS u ON u.id = fa.user_id");
        query.push_str(&format!(" WHERE u.username = '{}'", username_favorited_by));
    }

    if let Some(ref tag) = params.tag {
        query.push_str(" INNER JOIN article_tags AS at ON at.article_id = a.id");
        query.push_str(&format!(" WHERE at.tag_name = '{}'", tag));
    }

    let limit = std::cmp::min(params.limit.unwrap_or(20), 100) as i64;
    let offset = params.offset.unwrap_or(0) as i64;

    query.push_str(" ORDER BY a.created_at DESC");
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    let matched_articles: Vec<Article> = sqlx::query_as(&query)
        .fetch_all(pool)
        .await?;

    let user_id: Option<Uuid> = None; // Replace with the actual user_id
    let response = get_article_list_response(matched_articles, user_id, &pool).await;

    match response {
        Ok(article_list_response) => {
            Ok(HttpResponse::Ok().json(article_list_response))
        }
        Err(e) => {
            Err(e)
        }
    }
}

/// Create an article
#[utoipa::path(
    post,
    path = "/api/v1/articles/{username}",
    tag = "articles",
    responses(
        (status = 201, description = "Created", body = CreateArticle),
        (status = 400, description = "Bad request")
    ),
    params(
        ("username" = String, Path, description = "Username of a user"),
    ),
    request_body = CreateArticle
)]
pub async fn create_article(
    (form, username, pool): (web::Json<CreateArticle>, web::Path<UserForArticle>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    let article_data = form.into_inner();
    let user_info = username.into_inner();

    // Validate the user input
    let article_validation_result = article_data.validate();
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
    let author= sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1 LIMIT 1",
        &user_info.username
    )
    .fetch_one(pool)
    .await
    .map_err(|_| {
        AppError::InternalServerError
    })?;

    // Generating the Uuid here since it will help make a unique slug
    // This is for when some articles may have similar titles such that they generate the same slug
    let new_article_id = Uuid::new_v4();
    let slug = generate_slug(&new_article_id, &article_data.title);

    let new_article = NewArticle {
        id: new_article_id,
        author_id: author.id,
        slug,
        title: article_data.title,
        description: article_data.description,
        body: article_data.body,
    };

    // Create a query and bind parameters
    let query = sqlx::query("INSERT INTO articles (id, author_id, slug, title, description, body) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&new_article.id)
        .bind(&new_article.author_id)
        .bind(&new_article.slug)
        .bind(&new_article.title)
        .bind(&new_article.description)
        .bind(&new_article.body);

    // Execute the query on the pool
    match query.execute(pool).await {
        Ok(_) => {
            let _ = replace_tags(new_article.id, article_data.tag_list, pool).await?;

            // Fetch the article response after it's created
            let article_response = get_article_response(new_article.slug, Some(new_article.author_id), pool).await?;

            // Return the article response as an HTTP response
            Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(article_response))
        }
        Err(err) => {
            let custom_err: AppError = err.into();
            Ok(custom_err.error_response())
        }
    }
}

/// Return article feed
///
/// It is quite similar to get all articles, but this one supposed to be use to get article from someone you've followed and an options `limit` and `offset`
#[utoipa::path(
    get,
    path = "/api/v1/articles/feed/{username}",
    tag = "articles",
    responses(
        (status = 200, description = "Success", body = CreateArticle),
        (status = 400, description = "Bad request")
    ),
    params(
        ("username" = String, Path, description = "Username of a user"),
        ("limit" = i64, Query, description = "Limit article output", minimum = 20),
        ("offset" = i64, Query, description = "Offset article output", minimum = 0)
    ),
)]
pub async fn get_articles_feed(
    (params, username, pool): (web::Query<FeedParams>, web::Path<UserForArticle>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    // TODO: use query string for limit & offset payload
    // Access the PgPool from the Data container
    let pool = pool.get_ref();

    let user_info = username.into_inner();

    // Validate the user input
    let user_validation_result = user_info.validate();
    if let Err(validation_errors) = user_validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

    // Fetch the current user
    let author= sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1 LIMIT 1",
        &user_info.username
    )
    .fetch_one(pool)
    .await
    .map_err(|_| {
        AppError::InternalServerError
    })?;

    let limit = std::cmp::min(params.limit.unwrap_or(20), 100) as i64;
    let offset = params.offset.unwrap_or(0) as i64;

    let user_id = author.id;

    let following_ids: Vec<Uuid> = sqlx::query_as::<_, (Uuid,)>("SELECT user_id FROM followers WHERE follower_id = $1")
        .bind(&user_id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|(id,)| id)
        .collect();

    // Query articles based on the following_ids.
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, created_at FROM articles WHERE author_id = ANY($1) ORDER BY created_at DESC LIMIT $2 OFFSET $3"
    )
    .bind(&following_ids)
    .bind(&limit)
    .bind(&offset);

    // Execute the query on the pool
    match articles.fetch_all(pool).await {
        Ok(res) => {
            let article_response = get_article_list_response(res, Some(user_id), pool).await?;

            // Return the article response as an HTTP response
            Ok(HttpResponse::Ok().json(article_response))
        }
        Err(err) => {
            let custom_err: AppError = err.into();
            Ok(custom_err.error_response())
        }
    }
}

/// Return a specific article
#[utoipa::path(
    get,
    path = "/api/v1/articles/data/{slug}",
    tag = "articles",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("slug" = String, Path, description = "an article slug"),
    )
)]
pub async fn get_articles_by_slug(
    (path, pool): (web::Path<ArticlePath>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    // Access the PgPool from the Data container
    let pool = pool.get_ref();

    let article_response = get_article_response(path.slug.to_string(), None, pool).await?;

    // Return the article response as an HTTP response
    Ok(HttpResponse::Ok().json(article_response))
}

/// Update an article
#[utoipa::path(
    put,
    path = "/api/v1/articles/data/{slug}",
    tag = "articles",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("slug" = String, Path, description = "an article slug"),
        ("username" = String, Query, description = "Username of a user"),
    ),
    request_body = UpdateArticleOuter
)]
pub async fn update_articles_by_slug(
    (path, username, form, pool): (web::Path<ArticlePath>, web::Query<UserForArticle>, web::Json<UpdateArticleOuter>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    let update_article = form.into_inner().article;
    let user_info = username.into_inner();

    // Validate the user input
    let validation_result = update_article.validate();
    if let Err(validation_errors) = validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

    // Validate the user input
    let validation_result = user_info.validate();
    if let Err(validation_errors) = validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

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

    let (article_id, article_author_id, article_slug): (Uuid, Uuid, String) = sqlx::query_as::<_, (Uuid, Uuid, String)>("SELECT id, author_id, slug FROM articles WHERE slug = $1")
        .bind(&path.slug)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            AppError::InternalServerError
        })?;

    if user.id != article_author_id {
        return Err(AppError::Forbidden(serde_json::json!({
            "error": "user is not the author of article in question",
        })));
    }

    let slug = match &update_article.title {
        Some(title) => Some(generate_slug(&article_id, &title)),
        None => None,
    };

    let article_change = ArticleChange {
        slug,
        title: update_article.title,
        description: update_article.description,
        body: update_article.body,
    };

    let article = sqlx::query("UPDATE articles SET slug = $1, title = $2, description = $3, body = $4 WHERE id = $5 RETURNING *")
        .bind(&article_change.slug)
        .bind(&article_change.title)
        .bind(&article_change.description)
        .bind(&article_change.body)
        .bind(&article_id);

    match article.fetch_one(pool).await {
        Ok(res) => {
            let record_article_id: Uuid = res.get("id");
            let record_author_id: Uuid = res.get("author_id");

            let _ = match update_article.tag_list {
                Some(tags) => {
                    let inserted_tags = replace_tags(record_article_id, tags, pool).await?;

                    inserted_tags
                        .iter()
                        .map(|article_tag| article_tag.tag_name.to_owned())
                        .collect::<Vec<String>>()
                }
                None => select_tags_on_article(res.get("id"), pool).await?,
            };
            let article_response: ArticleResponse = get_article_response(article_slug, Some(record_author_id), pool).await?;

            // Return the article response as an HTTP response
            Ok(HttpResponse::Ok().json(article_response))
        },
        Err(err) => {
            // Convert SQLx error into custom AppError enum
            let custom_err: AppError = err.into();
            Ok(custom_err.error_response())
        }
    }
}

/// Delete an article
#[utoipa::path(
    delete,
    path = "/api/v1/articles/data/{slug}",
    tag = "articles",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("slug" = String, Path, description = "an article slug"),
        ("username" = String, Query, description = "Username of a user"),
    )
)]
pub async fn delete_articles_by_slug(
    (path, username, pool): (web::Path<ArticlePath>, web::Query<UserForArticle>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    let user_info = username.into_inner();

    // Validate the user input
    let validation_result = user_info.validate();
    if let Err(validation_errors) = validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

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

    let (article_id,article_author_id): (Uuid, Uuid) = sqlx::query_as::<_, (Uuid, Uuid)>("SELECT id, author_id, slug FROM articles WHERE slug = $1")
        .bind(&path.slug)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            AppError::InternalServerError
        })?;

    if user.id != article_author_id {
        return Err(AppError::Forbidden(serde_json::json!({
            "error": "user is not the author of article in question",
        })));
    }

    delete_tags(article_id, pool).await?;

    delete_favorites(article_id, pool).await?;

    let query = sqlx::query("DELETE FROM articles WHERE id = $1")
        .bind(&article_id);

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

/// Favorite an article
#[utoipa::path(
    post,
    path = "/api/v1/articles/favorite/{slug}",
    tag = "articles",
    responses(
        (status = 201, description = "Success"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("slug" = String, Path, description = "an article slug"),
        ("username" = String, Query, description = "Username of a user"),
    )
)]
pub async fn favorite_articles_by_slug(
    (path, username, pool): (web::Path<ArticlePath>, web::Query<UserForArticle>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    let user_info = username.into_inner();

    // Validate the user input
    let validation_result = user_info.validate();
    if let Err(validation_errors) = validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

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

    let (article_id,article_slug): (Uuid, String) = sqlx::query_as::<_, (Uuid, String)>("SELECT id, slug FROM articles WHERE slug = $1")
        .bind(&path.slug)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            AppError::InternalServerError
        })?;


    let favorite_article = NewFavoriteArticle {
        user_id: user.id,
        article_id: article_id
    };

    let query = sqlx::query("INSERT INTO favorite_articles VALUES ($1, $2)")
        .bind(&favorite_article.user_id)
        .bind(&favorite_article.article_id);

    // Execute the query on the pool
    match query.execute(pool).await {
        Ok(_) => {
            let success_response = serde_json::json!({
                "message": "Record created successfully",
            });

            // Fetch the article response after it's created
            get_article_response(article_slug, Some(user.id), pool).await?;

            // Return a successful response
            Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(success_response))
        }
        Err(err) => {
            // Convert SQLx error into custom AppError enum
            let custom_err: AppError = err.into();
            Ok(custom_err.error_response())
        }
    }
}

/// Unfavorite an article
#[utoipa::path(
    delete,
    path = "/api/v1/articles/favorite/{slug}",
    tag = "articles",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("slug" = String, Path, description = "an article slug"),
        ("username" = String, Query, description = "Username of a user"),
    )
)]
pub async fn unfavorite_articles_by_slug(
    (path, username, pool): (web::Path<ArticlePath>, web::Query<UserForArticle>, web::Data<PgPool>)
) -> Result<HttpResponse, AppError> {
    let user_info = username.into_inner();

    // Validate the user input
    let validation_result = user_info.validate();
    if let Err(validation_errors) = validation_result {
        return Ok(validation_errors_response(&validation_errors));
    }

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

    let (article_id,article_slug): (Uuid, String) = sqlx::query_as::<_, (Uuid, String)>("SELECT id, slug FROM articles WHERE slug = $1")
        .bind(&path.slug)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            AppError::InternalServerError
        })?;

    let query = sqlx::query("DELETE FROM favorite_articles WHERE user_id = $1 AND article_id = $2")
        .bind(&user.id)
        .bind(&article_id);

    match query.execute(pool).await {
        Ok(res) => {
            if res.rows_affected() > 0 {
                let success_response = serde_json::json!({
                    "message": "Record deleted successfully",
                });

                // Fetch the article response after it's created
                get_article_response(article_slug, Some(user.id), pool).await?;

                // Return a successful response
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
async fn get_article_list_response(
    articles: Vec<Article>,
    user_id: Option<Uuid>,
    pool: &PgPool,
) -> Result<ArticleListResponse, AppError> {
    let futures = articles
        .iter()
        .map(|article| {
            let article_slug = article.slug.to_owned();

            async move {
                match get_article_response(article_slug, user_id, pool).await {
                    Ok(response) => Ok(response.article),
                    Err(e) => Err(e),
                }
            }
        });

    let results: Vec<Result<ArticleResponseInner, AppError>> = join_all(futures).await;

    let articles: Result<Vec<ArticleResponseInner>, AppError> = results
        .into_iter()
        .collect();

    match articles {
        Ok(article_list) => {
            Ok(ArticleListResponse {
                articles_count: article_list.len(),
                articles: article_list,
            })
        }
        Err(e) => Err(e),
    }
}

async fn get_article_response(
    slug: String,
    user_id: Option<Uuid>,
    pool: &PgPool,
) -> Result<ArticleResponse, AppError> {
    let data: ArticleAndAuthor = sqlx::query_as("
        SELECT
            articles.*, users.* FROM articles
        INNER JOIN
            users ON articles.author_id = users.id
        WHERE
            articles.slug = $1"
    )
        .bind(slug)
        .fetch_one(pool)
        .await?;

    let (favorited, following) = match user_id {
        Some(user_id) => get_favorited_and_following(data.article.id, data.author.id, user_id, pool).await?,
        None => (false, false),
    };

    let favorites_count = get_favorites_count(data.article.id, pool).await?;
    let tags = select_tags_on_article(data.article.id, pool).await?;

    Ok(ArticleResponse {
        article: ArticleResponseInner {
            slug: data.article.slug,
            title: data.article.title,
            description: data.article.description,
            body: data.article.body,
            tag_list: tags,
            created_at: CustomDateTime(data.article.created_at),
            updated_at: CustomDateTime(data.article.updated_at),
            favorited,
            favorites_count,
            author: ProfileResponseInner {
                username: data.author.username,
                bio: data.author.bio,
                following,
            },
        },
    })
}

async fn get_favorites_count(article_id: Uuid, pool: &PgPool) -> Result<usize, AppError> {
    let favorites_count: i64 = sqlx::query!(
        "SELECT COUNT(*) FROM favorite_articles WHERE article_id = $1",
        article_id
    )
    .fetch_one(pool)
    .await?
    .count.unwrap_or(0);

    Ok(favorites_count as usize)
}

async fn select_tags_on_article(article_id: Uuid, pool: &PgPool) -> Result<Vec<String>, AppError> {
    let tags: Vec<String> = sqlx::query!(
        "SELECT tag_name FROM article_tags WHERE article_id = $1",
        article_id
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| row.tag_name)
    .collect();

    Ok(tags)
}

async fn get_favorited_and_following(
    article_id: Uuid,
    author_id: Uuid,
    user_id: Uuid,
    pool: &PgPool,
) -> Result<(bool, bool), AppError> {
    let query = sqlx::query_as(r#"
        SELECT
            users.id,
            favorite_articles.user_id,
            followers.follower_id
        FROM
            users
        LEFT JOIN favorite_articles
            ON favorite_articles.user_id = users.id AND favorite_articles.article_id = $1
        LEFT JOIN followers
            ON followers.follower_id = users.id AND followers.user_id = $2
        WHERE users.id = $3
    "#)
    .bind(&article_id)
    .bind(&author_id)
    .bind(&user_id);

    let result: (Option<Uuid>, Option<Uuid>) = query.fetch_one(pool).await?;
    let (_user_id, favorite_id, follow_id) = (Uuid::new_v4(), result.0, result.1); // Replace Uuid::new_v4() with an appropriate default Uuid

    Ok((favorite_id.is_some(), follow_id.is_some()))
}

impl<'r> FromRow<'r, PgRow> for Article {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Article {
            id: row.try_get("id")?,
            author_id: row.try_get("author_id")?,
            slug: row.try_get("slug")?,
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            body: row.try_get("body")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl<'r> FromRow<'r, PgRow> for ArticleAndAuthor {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(ArticleAndAuthor {
            article: Article {
                id: row.try_get("id")?,
                author_id: row.try_get("author_id")?,
                slug: row.try_get("slug")?,
                title: row.try_get("title")?,
                description: row.try_get("description")?,
                body: row.try_get("body")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            },
            author: User {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                email: row.try_get("email")?,
                password: row.try_get("password")?,
                bio: row.try_get("bio")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            },
        })
    }
}

fn generate_slug(uuid: &Uuid, title: &str) -> String {
    format!("{}-{}", to_blob(uuid), sluggify(title, None))
}

async fn replace_tags<I>(
    article_id: Uuid,
    tags: I,
    pool: &PgPool,
) -> Result<Vec<ArticleTag>, AppError>
where
    I: IntoIterator<Item = String>,
{
    delete_tags(article_id, pool).await?;

    let tag_results: Result<Vec<ArticleTag>, AppError> = Ok(try_join_all(tags
        .into_iter()
        .map(|tag_name| {
            let article_id = article_id;
            let tag_name = tag_name.to_string();
            let pool = pool.clone();

            async move {
                let result = add_tag(article_id, &tag_name, &pool).await?;
                Result::<_, AppError>::Ok(result)
            }
            .boxed()
        }))
        .await
        .map_err(|_| {
            // Handle the error here
            AppError::InternalServerError
        })?);

    tag_results
}

async fn add_tag<T>(article_id: Uuid, tag_name: T, pool: &PgPool) -> Result<ArticleTag, AppError>
where
    T: ToString,
{
    let tag_name = tag_name.to_string();

    let result = sqlx::query_as!(
        ArticleTag,
        r#"
        INSERT INTO article_tags (article_id, tag_name)
        VALUES ($1, $2)
        ON CONFLICT (article_id, tag_name)
        DO NOTHING
        RETURNING *
        "#,
        article_id,
        tag_name
    )
    .fetch_one(pool)
    .await
    .map_err(|_| {
        // eprintln!("SQLx Error: {:?}", err);
        AppError::InternalServerError
    })?;

    Ok(result)
}

async fn delete_tags(article_id: Uuid, pool: &PgPool) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM article_tags WHERE article_id = $1", article_id)
        .execute(pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok(())
}

async fn delete_favorites(article_id: Uuid, pool: &PgPool) -> Result<(), AppError> {
    let _ = sqlx::query("DELETE FROM favorite_articles WHERE article_id = $1")
        .bind(&article_id)
        .execute(pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok(())
}
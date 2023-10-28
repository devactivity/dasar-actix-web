use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use sqlx::{FromRow, postgres::PgRow, Row};

use super::{ProfileResponseInner, CustomDateTime, User};

#[derive(Debug)]
pub struct Comment {
    pub id: i32,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct NewComment {
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub body: String,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct AddComment {
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub body: String,
}

#[derive(Debug)]
pub struct AddCommentOuter {
    pub slug: String,
    pub comment: AddComment,
}

#[derive(Debug)]
pub struct GetComments {
    pub slug: String,
}

#[derive(Debug)]
pub struct DeleteComment {
    pub slug: String,
    pub comment_id: i32,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub comment: CommentResponseInner,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentResponseInner {
    pub id: i32,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
    pub body: String,
    pub author: ProfileResponseInner,
}

#[derive(Debug, Serialize)]
pub struct CommentListResponse {
    pub comments: Vec<CommentResponseInner>,
}

#[derive(Debug, Deserialize)]
pub struct ArticleCommentPath {
    pub slug: String,
    pub comment_id: i32,
}

#[derive(Debug)]
pub struct CommentAndCommenter {
    pub comment: Comment,
    pub commenter: User,
}

impl<'r> FromRow<'r, PgRow> for CommentAndCommenter {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(CommentAndCommenter {
            comment: Comment {
                id: row.try_get("id")?,
                article_id: row.try_get("article_id")?,
                user_id: row.try_get("user_id")?,
                body: row.try_get("body")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?
            },
            commenter: User {
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
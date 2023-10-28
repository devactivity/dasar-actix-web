use uuid::Uuid;
use serde::{Deserialize, Serialize, Serializer};
use utoipa::ToSchema;
use validator::Validate;
use chrono::NaiveDateTime;
use regex::Regex;
use lazy_static::lazy_static;
use crate::schemas::users_schema::User;

use super::ProfileResponseInner;

#[derive(Debug, PartialEq, ToSchema)]
pub struct CustomDateTime(pub NaiveDateTime);

impl Serialize for CustomDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = self.0.format("%Y-%m-%dT%H:%M:%S.%3fZ");
        serializer.serialize_str(&s.to_string())
    }
}

#[derive(Debug)]
pub struct Article {
    pub id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ArticlesParams {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ArticleResponse {
    pub article: ArticleResponseInner,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ArticleResponseInner {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
    pub favorited: bool,
    pub favorites_count: usize,
    pub author: ProfileResponseInner,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ArticleListResponse {
    pub articles: Vec<ArticleResponseInner>,
    pub articles_count: usize,
}

#[derive(Debug)]
pub struct ArticleAndAuthor {
    pub article: Article,
    pub author: User,
}

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[_0-9a-zA-Z]{3,}$").unwrap();
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct UserForArticle {
    #[validate(
        length(
            min = 3,
            max = 20,
            message = "fails validation - must be 3-20 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: String,
}

#[derive(Debug)]
pub struct NewArticle {
    pub id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateArticle {
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub title: String,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub description: String,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub body: String,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub tag_list: Vec<String>,
}

#[derive(Debug)]
pub struct CreateArticleOuter {
    pub article: CreateArticle,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct FeedParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug)]
pub struct Follower {
    pub user_id: Uuid,
    pub follower_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ArticlePath {
    pub slug: String,
}

#[derive(Debug)]
pub struct GetArticle {
    pub slug: String,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticle {
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub title: Option<String>,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub description: Option<String>,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub body: Option<String>,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub tag_list: Option<Vec<String>>,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct UpdateArticleOuter {
    pub slug: String,
    pub article: UpdateArticle,
}

#[derive(Debug)]
pub struct ArticleChange {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug)]
pub struct FavoriteArticle {
    pub slug: String,
}

#[derive(Debug)]
pub struct UnfavoriteArticle {
    pub slug: String,
}
#[derive(Debug)]
pub struct NewFavoriteArticle {
    pub user_id: Uuid,
    pub article_id: Uuid,
}

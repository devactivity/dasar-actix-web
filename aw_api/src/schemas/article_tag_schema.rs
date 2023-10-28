use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub struct GetTags {}

#[derive(Serialize, ToSchema)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

#[derive(Debug, ToSchema)]
pub struct ArticleTag {
    pub article_id: Uuid,
    pub tag_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, ToSchema)]
pub struct NewArticleTag {
    pub article_id: Uuid,
    pub tag_name: String,
}

use super::schema::items;
use uuid::Uuid;
use serde::Deserialize;

#[derive(QueryableByName)]
pub struct Item {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub name: String
}

#[derive(Debug, Insertable)]
#[diesel(table_name = items)]
pub struct NewItem {
    pub id: Option<Uuid>,
    pub name: String
}


#[derive(Debug, Deserialize)]
pub struct CreateItemPayload {
    pub name: String
}

#[derive(Deserialize)]
pub struct UpdateItemRequest {
    pub item_name: String,
}


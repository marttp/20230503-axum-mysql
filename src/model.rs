use serde::{Serialize, Deserialize};
use chrono::{DateTime,Utc};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct NoteModel {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub published: i8,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteModelResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
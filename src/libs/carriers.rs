use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Debug, Clone, ToSql, FromSql)]
pub struct Carrier {
    pub id: i64,
    pub name: String,
    pub abbreviation: String,
    pub created_at: DateTime<Utc>,
}
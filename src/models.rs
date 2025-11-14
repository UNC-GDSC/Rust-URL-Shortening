use crate::schema::urls;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Url {
    pub id: i32,
    pub original_url: String,
    pub short_code: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = urls)]
pub struct NewUrl {
    pub original_url: String,
    pub short_code: String,
}

use chrono::NaiveDateTime;

use crate::models::like::Like;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Tweet {
    pub id: String,
    pub message: String,
    pub likes: Vec<Like>,
    pub created_at: NaiveDateTime,
}

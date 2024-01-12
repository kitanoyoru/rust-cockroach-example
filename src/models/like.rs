use chrono::NaiveDateTime;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Like {
    pub id: String,
    pub created_at: NaiveDateTime,
}

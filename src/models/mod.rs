use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Tweet {
    pub id: String,
    pub message: String,
    pub likes: Vec<Like>,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Like {
    pub id: String,
    pub created_at: NaiveDateTime,
}

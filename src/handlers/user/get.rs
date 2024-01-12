use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

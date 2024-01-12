use crate::handlers::user::get::UserResponse;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ListUserResponse(pub Vec<UserResponse>);

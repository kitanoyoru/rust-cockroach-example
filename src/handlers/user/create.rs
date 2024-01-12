use actix_web::web::{block, Data, Json};
use uuid::Uuid;
use validator::Validate;

use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::validate::validate;
use crate::models::user::{User, CreateUserDTO};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateUserRequest {
    #[validate(length(
        min = 10,
        message = "first_name is required and must be at least 10 characters"
    ))]
    pub first_name: String,

    #[validate(length(
        min = 10,
        message = "last_name is required and must be at least 10 characters"
    ))]
    pub last_name: String,

    #[validate(email(message = "email must be a valid"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateUserResponse {
    id: String,
}

pub fn create(
    pool: Data<PoolType>,
    params: Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, ApiError> {
    validate(&params)?;

    let new_user: User = CreateUserDTO {
        id: Uuid::new_v4().to_string(),
        first_name: params.first_name.to_string(),
        last_name: params.last_name.to_string(),
        email: params.email.to_string(),
        password: params.password.to_string(),
    }.into();
    
    let user =  
}

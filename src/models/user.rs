use chrono::NaiveDateTime;

use crate::database::PoolType;
use crate::errors::ApiError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateUserDTO {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub fn create(pool: &PoolType, new_user: &User) -> Result<String, ApiError> {
    use crate::schema::dsl::users;

    let conn = pool.get()?;
    diesel::insert_into(users).values(new_user);

    Ok(new_user.clone().into())
}

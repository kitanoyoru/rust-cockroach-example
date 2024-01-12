use diesel::prelude::*;

use crate::database::PoolType;
use crate::errors::ApiError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateUserDTO {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn create(pool: &PoolType, new_user: &User) -> Result<User, ApiError> {
        use crate::schema::users;

        let conn = pool.get()?;
        diesel::insert_into(users)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(conn)?;

        Ok(new_user.clone().into())
    }
}

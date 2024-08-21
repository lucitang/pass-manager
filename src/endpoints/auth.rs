use diesel::prelude::*;
use crate::{ApiError, guards::AuthKey, establish_connection, MyDatabase};
use crate::models::User;

#[get("/auth")]
pub async fn authenticate(conn: MyDatabase, auth_key: AuthKey) -> Result<&'static str, ApiError> {
    conn.run(move |c| authenticate_key(auth_key)).await
}

fn authenticate_key(auth_key: AuthKey ) -> Result<&'static str, ApiError> {
    Ok(if check_user_exists(auth_key)? {
        "1"
    } else {
        "0"
    })
}

pub fn check_user_exists(auth_key: AuthKey) ->  Result<bool, ApiError>{
    // Do something with connection, return some data.
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let user_list = users
        .filter(key.eq(String::from(auth_key)))
        .select(User::as_select())
        .load(connection)
        .map_err(|_| {
            error!("Failed to read database");

            ApiError::DatabaseRead
        })?;
    if user_list.len() > 1 {
        error!("INTERNAL ERROR: Two users cannot have identical authentication keys");

        Err(ApiError::InternalError)
    } else {
        Ok(user_list.len() == 1)
    }
}

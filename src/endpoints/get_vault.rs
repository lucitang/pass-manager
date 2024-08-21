use diesel::prelude::*;
use crate::{ApiError, guards::AuthKey, establish_connection, MyDatabase};
use crate::models::User;

#[get("/get_vault")]
pub async fn retrieve_vault(conn: MyDatabase, auth_key: AuthKey) -> Result<String, ApiError>{
    conn.run(move |c| read_vault_from_db(auth_key)).await

}

fn read_vault_from_db(
    auth_key: AuthKey,
) -> Result<String, ApiError> {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let mut user_list = users
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
        let user = user_list.pop().ok_or_else(||{
            error!("Attempted to request vault from user with authentication key not in database");

            ApiError::UserNoExists
        })?;
        Ok(user.vault)
    }
}
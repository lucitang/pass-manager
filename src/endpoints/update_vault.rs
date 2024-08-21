
use diesel::prelude::*;
use crate::schema::users::dsl::*;

use crate::{ApiError, guards::{AuthKey, Vault}, establish_connection,models::User, MyDatabase };
use crate::guards::Email;

#[get("/update_vault")]
pub async fn set_new_vault(conn:MyDatabase, auth_key: AuthKey, _vault: Vault) -> Result<String, ApiError>{
    conn.run(move |c| update_vault_in_db(auth_key, _vault)).await
}

fn update_vault_in_db(auth_key: AuthKey, new_vault:Vault) -> Result<String, ApiError>{
    let connection = &mut establish_connection();
    if check_user_exists(auth_key)?{
        diesel::update(users.filter(key.eq(String::from(auth_key))))
            .set(vault.eq(new_vault.0))
            .returning(User::as_returning())
            .get_result(connection)
            .map_err(|_| {
                ApiError::DatabaseWrite
            })
            .map(|_| {
                info!("Successfully registered new user in database");
                String::from("Success")
            })

    }
    else {
        Err(ApiError::UserNoExists)
    }
}

fn check_user_exists(auth_key: AuthKey) ->  Result<bool, ApiError>{
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

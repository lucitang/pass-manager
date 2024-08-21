use diesel::prelude::*;
// use log::{info, error, trace};
use crate::{ApiError, guards::AuthKey, guards::Email, establish_connection, models::User, MyDatabase, guards::Vault};
use crate::models::NewUser;

#[get("/register")]
pub async fn register_user(conn: MyDatabase, email: Email, auth_key:AuthKey, vault: Vault) -> Result<String, ApiError>{
    conn.run(move |c| register_new_user(email, auth_key, vault)).await
}

fn register_new_user( email: Email, auth_key: AuthKey, vault:Vault) -> Result<String, ApiError> {
    let connection = &mut establish_connection();
    trace!("Registering new user");
    use crate::schema::users;
    if (check_email_exists(&email))?{
        error!("User already exists in database");

        Err(ApiError::UserExists)
    }
    else{
        let new_user = NewUser{
            email: email.0,
            key: auth_key.into(),
            vault: vault.0,
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(connection)
            .map_err(|_| {
                error!("Failed to write new user to database");
                ApiError::DatabaseWrite
            })
            .map(|_| {
                info!("Successfully registered new user in database");
                String::from("Success")
            })
    }
}

fn check_email_exists(check_email: &Email) -> Result<bool, ApiError>{
    let connection = &mut establish_connection();
    use crate::schema::users::dsl::*;
    let mut user_list = users
        .filter(email.eq(&check_email.0))
        .select(User::as_select())
        .load(connection)
        .map_err(|_| {
            error!("Failed to read Database");
            ApiError::DatabaseRead
        })?;
    if (user_list.len() > 1){
        error!("INTERNAL ERROR: Two users cannot have identical emails");
        Err(ApiError::InternalError)
    }else {
        Ok(user_list.len() == 1)
    }
}
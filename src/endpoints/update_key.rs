use diesel::prelude::*;
use crate::schema::users::dsl::*;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

use crate::{ApiError, guards::{AuthKey, Vault}, establish_connection,models::User, MyDatabase };

#[get("/update_key")]
pub async fn set_new_key(conn:MyDatabase, old_auth_key:AuthKey, new_auth_key:NewAuthKey, _vault:Vault) ->Result<String, ApiError>{
    conn.run(move |c| update_authkey_in_db(old_auth_key, new_auth_key, _vault)).await
}


fn update_authkey_in_db(__old_auth_key:AuthKey, __new_auth_key:NewAuthKey, __vault:Vault) -> Result<String, ApiError>{
    let connection = &mut establish_connection();
    if check_user_exists(__old_auth_key)? {
        diesel::update(users.filter(key.eq(String::from(__old_auth_key))))
            .set((key.eq(String::from(__new_auth_key)), vault.eq(__vault.0)))
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


#[derive(Clone, Copy)]
pub struct NewAuthKey(pub [u8; 32]);

impl From<NewAuthKey> for String {
    fn from(__key: NewAuthKey) -> Self {
        hex::encode(__key.0)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for NewAuthKey {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("x-new-auth-key") {
            Some(key_str) => {
                let mut key_bytes = [0; 32];

                if hex::decode_to_slice(key_str, &mut key_bytes).is_err() {
                    Outcome::Error((Status::BadRequest, ApiError::AuthKeyInvalid))
                } else {
                    Outcome::Success(NewAuthKey(key_bytes))
                }
            }
            None => Outcome::Error((Status::BadRequest, ApiError::AuthKeyMissing)),
        }
    }
}
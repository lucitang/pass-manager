#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

use diesel::prelude::*;

use rocket_sync_db_pools::{database};
use dotenvy::dotenv;
use std::env;

use rocket::http::{Status, ContentType};
use rocket::request::{Request, FromRequest};
use rocket::response::{self, Responder};
use rocket::Response;

mod schema;
mod models;
mod endpoints;
mod guards;

use endpoints::*;
const LOG_PATH: &str = "logs/password_manager.log";

//represents error which is returned when there is an internal error or bad request
#[derive(Debug)]
pub enum ApiError{
    AuthKeyMissing,
    AuthKeyInvalid,
    EmailMissing,
    EmailInvalid,
    VaultMissing,
    VaultInvalid,
    DatabaseWrite,
    DatabaseRead,
    InternalError,
    UserExists,
    UserNoExists,
}

impl From<ApiError> for String {
    fn from(e: ApiError) -> Self {
        String::from(match e {
            ApiError::AuthKeyMissing => "Authentication key missing",
            ApiError::AuthKeyInvalid => "Authentication key invalid",
            ApiError::EmailMissing => "Email missing",
            ApiError::EmailInvalid => "Email invalid",
            ApiError::VaultMissing => "Vault missing",
            ApiError::VaultInvalid => "Vauld invalid",
            ApiError::DatabaseWrite => "failed to write to Database",
            ApiError::DatabaseRead => "failed to read to Database",
            ApiError::InternalError=> "Internal Server Error",
            ApiError::UserExists => "Users already exists in the database",
            ApiError::UserNoExists => "User does not exist in database",
        })
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let s: String = self.into();

        Response::build_from(s.respond_to(req)?)
            .status(Status::BadRequest)
            .header(ContentType::Text)
            .ok()
    }
}




#[catch(400)]
fn bad_request() -> &'static str {
    "Bad request"
}

//wrapper struct representing Databas Connection
#[database("password_manager")]
struct MyDatabase(PgConnection);

//connects to the database using the uri from the env file;
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}



#[launch]
fn rocket() -> _ {
    let log_file = std::fs::File::options()
        .append(true)
        .create(true)
        .open(LOG_PATH)
        .expect("Failed to open logging file");

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
        ]
    ).expect("Failed to create logging system ");


    rocket::build().
        attach(MyDatabase::fairing())
        .register("/api", catchers![bad_request])
        .mount("/api", routes![authenticate, register_user, retrieve_vault, set_new_vault, set_new_key])
}



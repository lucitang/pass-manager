use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct User{
    pub id: i32,
    pub email: String,
    pub key: String,
    pub vault: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser{
    pub email: String,
    pub key: String,
    pub vault:String,
}
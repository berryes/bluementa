use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {

    pub id: String,
    pub permission_level: i8,
    pub display_name: String,
    pub username: String,
    pub password: String,

}
use rocket::FromForm;

#[derive(Debug, FromForm)]
pub struct FormUser {
    pub username: String,
    pub password: String,
    pub display_name: String,
}


use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a String,
    pub premission_level: &'a i8,
    pub display_name: &'a String,
    pub username: &'a String,
    pub password: &'a String,
}


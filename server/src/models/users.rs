use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};

#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub permission_level: i8,
    pub username: String,
    pub password: String,

}

use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub premission_level: &'a i8,
    pub username: &'a String,
    pub password: &'a String,
}





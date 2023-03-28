use diesel::prelude::*;
use rocket::request::{self, Outcome, Request, FromRequest};
use rocket::request::Request;
use rocket::data::{self, Data, FromData};

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
    pub id: &'a String,
    pub premission_level: &'a i8,
    pub username: &'a String,
    pub password: &'a String,
}





pub struct UserRegister<'r> {
    pub username: &'r String,
    pub password: &'r String,
}

#[derive(Debug)]
pub enum UserRegisterError{
    FieldEmpty,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromData<'r> for UserRegister<'r> {
    type Error = UserRegisterError;
    

    // check incoming data
    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {



    }
}
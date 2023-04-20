use diesel::{mysql::MysqlConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use models::users::{FormUser, User};
use schema::users::{self, username};
use std::env;

pub mod schema;
pub mod models;


pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

}

pub fn get_user(formuser: FormUser, connection: &mut MysqlConnection) -> QueryResult<User> 
{
    users::table.filter( username.eq(formuser.username) ).get_result::<User>( connection )

}
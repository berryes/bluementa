use diesel::internal::table_macro::SelectStatement;
use rocket::form::Form;
use server::schema::users::id;
use server::schema::users::username;
use uuid::Uuid;
use diesel::RunQueryDsl;
use diesel::prelude::*;
#[macro_use] extern crate rocket;
use server::{establish_connection,  schema::{users}, models::users::{NewUser, User, FormUser} };

#[post("/register", data = "<user>")]
fn new_user(user: Form<FormUser>) { 

    let connection = &mut establish_connection();

    let user_data = NewUser { 
        id: &format!("{}",Uuid::new_v4()),
        premission_level: &1,
        display_name: &String::from(user.display_name.clone()),
        username: &String::from(user.username.clone()),
        password: &String::from(user.password.clone())
    };

    diesel::insert_into(users::table)
    .values(&user_data)
    .execute(connection)
    .expect("asd");
 }



#[launch]
fn rocket() -> _ {    
    rocket::build().mount("/",
    routes![
        new_user
        ])

}

use rocket::form::Form;
use server::{establish_connection,  schema::{users, self}, models::users::{NewUser, User} };
use diesel::RunQueryDsl;
use uuid::Uuid;
#[macro_use] extern crate rocket;

#[derive(FromForm, Debug)]
struct RegisterUser<> {
    pub username: String,
    pub password: String,
}

#[post("/register", data = "<user>")]
fn new_user(user: Form<RegisterUser>) { 

    let connection = &mut establish_connection();


    let user_data = NewUser { 
        id: &format!("{}",Uuid::new_v4()),
        premission_level: &1,
        username: &String::from(user.username.clone()),
        password: &String::from(user.password.clone())
    };

    diesel::insert_into(users::table)
    .values(user_data)
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

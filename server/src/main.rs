use rocket::form::Form;
use server::{establish_connection,  schema::{users, self}, models::users::{NewUser, User, UserRegister} };
use diesel::RunQueryDsl;

#[macro_use] extern crate rocket;

#[ post("/register",  format = "application/json", data = "<register_data>") ]
fn new_user(register_data: Json<UserRegister<'_>> ) -> String {

    let connection = &mut establish_connection();

    let new_post = NewUser { 
        id: &String::from("asdasdas"),
        premission_level: &1,
        username: &String::from(name),
        password: &String::from("asd")
     };

    diesel::insert_into(users::table)
    .values(new_post)
    .execute(connection)
    .expect("asd");


    return String::from("ok")
}


#[launch]
fn rocket() -> _ {    
    rocket::build().mount("/", routes![new_user])

}

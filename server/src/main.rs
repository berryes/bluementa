use server::{establish_connection,  schema::users, models::users::NewUser };
use diesel::RunQueryDsl;

#[macro_use] extern crate rocket;

#[get("/new/user/<name>")]
fn new_user(name: &str ) -> String {

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

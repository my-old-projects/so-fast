#[macro_use]
extern crate rocket;

mod controllers;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![controllers::home::index])
}

#[macro_use]
extern crate rocket;

mod controllers;
pub use controllers::home;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![controllers::home::index])
}

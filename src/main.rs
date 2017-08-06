#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate padme;

#[cfg(test)]
mod tests;
mod handler;
mod padding;
mod params;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api/", routes![handler::leftpad, handler::leftpad_right])
}

fn main() {
    rocket().launch();
}

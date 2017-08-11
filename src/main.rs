#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate handlebars;

pub mod blog;

use std::fs::File;

fn main() {
	rocket::ignite().mount("/", routes![blog::index, blog::index_page, blog::post]).launch();
}

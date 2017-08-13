#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate handlebars;
extern crate postgres;
extern crate toml;
extern crate xdg;

pub mod blog;
pub mod db;

fn main() {
	let config = db::config::Config::load().unwrap();
	let db = db::DB::new(config.postgres.clone());

	rocket::ignite()
		.mount("/", routes![blog::index, blog::index_page, blog::post, blog::css, blog::scripts, blog::auth::auth, blog::submit_page, blog::submit_post])
		.manage(db)
		.manage(config)
	.launch();
}

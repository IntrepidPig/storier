#[get("/")]
fn index() -> Result<Response<'static>, Status> {
	index_page(0, 10)
}

#[get("/posts/<from>/<amount>")]
fn index_page(from: u32, amount: u32) -> Result<Response<'static>, Status> {
	let mut input = String::new();
	File::open(Path::new("/srv/blog/index.html")).unwrap().read_to_string(&mut input).unwrap();

	let mut navbar = String::new();
	File::open(Path::new("/srv/blog/templates/navbar.htmp")).unwrap().read_to_string(&mut navbar).unwrap();

	let mut posts: Vec<Post> = vec![Post { id: 1, safe_title: String::from("nice"), raw_title: String::from("raw_nice"), date: Utc::now(), text: String::from("Awful post"), author: Author { name: String::from("dope") } }];

	let context = json!({
		"navbar": navbar,
		"posts": posts
	});

	let mut reg = Handlebars::new();

	reg.register_helper("ulist", Box::new(|h: &Helper, _: &Handlebars, rc: &mut RenderContext| -> Result<(), RenderError> {

		let mut out = String::from("<ul>");

		for item in h.params() {
			out.push_str("<li>");

				let rendered = serde_json::from_str::<Post>(&item.value()[0].to_string()).expect("Failed to parse post from json").render();

			out.push_str(&rendered);
			out.push_str("</li>");
		};

		rc.writer.write(out.into_bytes().as_ref())?;

		Ok(())
	}));

	let output = reg.template_render(&input, &context).expect("Failed to render index template");

	Response::build()
		.header(ContentType::HTML)
		.sized_body(Cursor::new(output))
		.ok()
}

#[get("/post/<title>")]
fn post(title: String) -> String {
	serde_json::to_string(&Post { id: 1, safe_title: title.clone(), raw_title: title.clone(), date: Utc::now(), text: String::from("This would normally be longer"), author: Author { name: String::from("Dabnel") } }).unwrap()
}

#[macro_use]
use serde_derive;
use serde;
#[macro_use]
use serde_json;
use chrono::DateTime;
use chrono::Utc;
use rocket::response::{self, status, Response, Responder};
use rocket::Request;
use rocket::http::{ContentType, Status};
use handlebars::{Handlebars, Helper, RenderContext, RenderError};

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;

#[derive(Serialize, Deserialize)]
pub struct Post {
	pub id: u32,
	pub safe_title: String,
	pub raw_title: String,
	pub date: DateTime<Utc>,
	pub text: String,
	pub author: Author,
}

impl Post {
	fn get_context(&self) -> serde_json::value::Value {
		use chrono::prelude::*;
		json!({
			"title": self.raw_title,
			"day": self.date.format("").to_string(),
			"time": self.date.format("").to_string(),
			"text": self.text
		})
	}

	pub fn render(&self) -> String {
		let mut reg = Handlebars::new();
		let mut post_template = String::new();
		File::open(Path::new("/srv/blog/templates/post.htmp")).unwrap().read_to_string(&mut post_template).unwrap();

		let post_render = reg.template_render(&post_template, &self.get_context()).expect("Failed to render post");

		post_render
	}
}

#[derive(Serialize, Deserialize)]
pub struct Author {
	pub name: String,
}
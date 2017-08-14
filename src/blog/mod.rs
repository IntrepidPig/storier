pub mod auth;

use db::DB;
use db::config::Config;
use rocket::State;
use rocket::response::NamedFile;
use std::path::PathBuf;
use rocket::request::Form;
use rocket::response::Redirect;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

#[get("/")]
fn index(db: State<DB>, conf: State<Config>) -> Result<Response<'static>, Status> {
	index_page(0, 10, db, conf)
}

#[get("/posts/<from>/<amount>")]
fn index_page(from: u32, amount: u32, db: State<DB>, conf: State<Config>) -> Result<Response<'static>, Status> {
	let mut input = String::new();
	File::open(conf.root.join(Path::new("index.html"))).unwrap().read_to_string(&mut input).unwrap();

	let mut navbar = String::new();
	File::open(conf.root.join(Path::new("templates/navbar.htmp"))).unwrap().read_to_string(&mut navbar).unwrap();

	let posts: Vec<Post> = db.get_posts(from, amount);
	let posts: Vec<String> = posts.iter().map(|x| { x.render(&conf) }).collect();
	let mut posts_render = String::new();

	for post in &posts {
		posts_render.push_str(&post);
	}

	let context = json!({
		"navbar": navbar,
		"posts": posts_render
	});

	let reg = Handlebars::new();



	let output = reg.template_render(&input, &context).expect("Failed to render index template");

	Response::build()
		.header(ContentType::HTML)
		.sized_body(Cursor::new(output))
		.ok()
}

#[get("/post/<title>")]
fn post(title: String, db: State<DB>, conf: State<Config>) -> Result<Response<'static>, Status> {
	if let Some(post) = db.get_post_by_title(title) {
		let mut input = String::new();
		File::open(conf.root.join(Path::new("templates/single_post.htmp"))).unwrap().read_to_string(&mut input).unwrap();

		let mut navbar = String::new();
		File::open(conf.root.join(Path::new("templates/navbar.htmp"))).unwrap().read_to_string(&mut navbar).unwrap();
		let post_render = post.render(&conf);

		let reg = Handlebars::new();

		let context = json!({
			"navbar": navbar,
			"post": post_render,
		});

		let output = reg.template_render(&input, &context).expect("Failed to render single post template");

		Response::build()
			.header(ContentType::HTML)
			.sized_body(Cursor::new(output))
			.ok()
	} else {
		Err(Status::NotFound)
	}
}

#[get("/submit")]
fn submit_page(conf: State<Config>) -> Result<Response<'static>, Status> {
	let mut input = String::new();
	File::open(conf.root.join(Path::new("submit.html"))).unwrap().read_to_string(&mut input).unwrap();

	let mut navbar = String::new();
	File::open(conf.root.join(Path::new("templates/navbar.htmp"))).unwrap().read_to_string(&mut navbar).unwrap();

	let reg = Handlebars::new();

	let context = json!({
		"navbar": navbar
	});

	let output = reg.template_render(&input, &context).expect("Failed to render submit template");

	Response::build()
		.header(ContentType::HTML)
		.sized_body(Cursor::new(output))
		.ok()
}

#[post("/submit", data = "<submission>")]
fn submit_post(submission: Form<Submission>, db: State<DB>, config: State<Config>) -> Result<Redirect, Status> {
	fn hash(pass: &String) -> String {
		let mut sha = Sha256::new();
		sha.input_str(&pass);
		sha.result_str()
	}

	if hash(&submission.get().password) != config.passhash {
		Err(Status::Forbidden)
	} else {
		let post = submission.get().to_post();
		db.add_post(post);

		Ok(Redirect::to(config.root.join(Path::new("index.html")).to_str().unwrap()))
	}
}

#[get("/styles/<file..>")]
fn css(file: PathBuf, config: State<Config>) -> Option<NamedFile> {
	NamedFile::open(config.root.join(Path::new("styles/")).join(file)).ok()
}

#[get("/scripts/<file..>")]
fn scripts(file: PathBuf, conf: State<Config>) -> Option<NamedFile> {
	NamedFile::open(conf.root.join(Path::new("scripts/")).join(file)).ok()
}

use serde_json;
use chrono::DateTime;
use chrono::Local;
use rocket::response::Response;
use rocket::http::{ContentType, Status};
use handlebars::{Handlebars};

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;

#[derive(FromForm)]
struct Submission {
	pub title: String,
	pub body: String,
	pub password: String,
}

impl Submission {
	fn gen_safe_title(&self) -> String {
		let mut safe_title = String::new();

		for c in self.title.chars() {
			if c.is_alphanumeric() {
				safe_title.push(c);
			}
		}

		safe_title
	}

	fn to_post(&self) -> Post {
		let safe_title = self.gen_safe_title();
		let raw_title = self.title.clone();
		let date = Local::now();
		let text = self.body.clone();
		let author = Author { name: String::from("IntrepidPig") };

		Post { id: -1, safe_title: safe_title, raw_title: raw_title, date: date, text: text, author: author }
	}
}

#[derive(Serialize, Deserialize)]
pub struct Post {
	pub id: i32,
	pub safe_title: String,
	pub raw_title: String,
	pub date: DateTime<Local>,
	pub text: String,
	pub author: Author,
}

impl Post {
	fn get_context(&self) -> serde_json::value::Value {
		json!({
			"title": self.raw_title,
			"day": self.date.format("%a/%d %B %G").to_string(),
			"time": self.date.format("%l:%M %p").to_string(),
			"text": self.text
		})
	}

	pub fn render(&self, conf: &Config) -> String {
		let reg = Handlebars::new();
		let mut post_template = String::new();
		File::open(conf.root.join(Path::new("templates/post.htmp"))).unwrap().read_to_string(&mut post_template).unwrap();

		let post_render = reg.template_render(&post_template, &self.get_context()).expect("Failed to render post");

		post_render
	}
}

#[derive(Serialize, Deserialize)]
pub struct Author {
	pub name: String,
}
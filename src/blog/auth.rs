use rocket::State;
use rocket::http::{Cookies, Cookie};
use db::config::Config;

#[allow(unmounted_route)]
#[get("/auth")]
fn auth_page() {

}

#[allow(unmounted_route)]
#[post("/auth", data = "<password>")]
fn auth(password: String, config: State<Config>, mut cookies: Cookies) -> &'static str {
    if password == config.passhash {
        cookies.add_private(Cookie::new("password", password));
        println!("Got correct password, saved as cookie");
        "Authorized successfully"
    } else {
        println!("Got incorrect password, {}", password);
        "Bish get tf outta here"
    }
}
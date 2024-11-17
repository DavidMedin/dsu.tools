#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::serde::json::{self, Json};
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct LoginResponse {
    success: bool,
    message: String,
}

// v------ This is a Function Macro in Rust. It is some code that the Rocket library
//         Defines to make it easier to make a route.
#[post("/login", format = "application/json", data = "<user>")]
fn login(user: Json<User>) -> String {
    let response = LoginResponse {
        success: true,
        message: format!("Hello, {}!", user.username),
    };
    return json::to_string(&response).unwrap();
}

// If the login request doesn't have the corrent user information required,...
#[post("/login", rank = 2)]
fn failed_login() -> String {
    let response = LoginResponse {
        success: false,
        message: "Failed to login!".to_string(),
    };
    return json::to_string(&response).unwrap();
}

// 'rocket:main' is another macro that tells Rocket that this is our main function.
// While compiling, Rocket will modify this function using witchcraft.
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // This code starts up Rocket. It tells Rocket about our 'index()' function
    // we've defined above, and where to mount it.
    // Finally, it runs the server until the server is stopped.
    let _rocket = rocket::build()
        .mount("/", routes![login, failed_login])
        .mount("/", FileServer::from("../dsutools-frontend/dist/"))
        .launch()
        .await?;

    // At this point, the server has stopped.

    // In Rust, we have a Result type that is either Ok or an Error.
    // Here we are returning the Ok value.
    return Ok(());
}

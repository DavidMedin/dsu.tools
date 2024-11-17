#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct LoginResponse {
    token: String,
}

// v------ This is a Function Macro in Rust. It is some code that the Rocket library
//         Defines to make it easier to make a route.
#[post("/login", format = "application/json", data = "<user>")]
fn login(user: Json<User>) -> Result<Json<LoginResponse>, Status> {
    if user.username != "admin" || user.password != "Password1!" {
        return Err(Status::Unauthorized);
    }

    let response = LoginResponse {
        token: "A big long token".to_string(),
    };
    return Ok(Json(response));
}

// If the login request doesn't have the corrent user information required,...
#[post("/login", rank = 2)]
fn failed_login() -> Status {
    // TODO: pick better error codes.
    return Status::Unauthorized;
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

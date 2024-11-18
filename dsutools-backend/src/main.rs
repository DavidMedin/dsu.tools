#[macro_use]
extern crate rocket;

// Importing libraries ====================
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};

use rocket_db_pools::sqlx;
use rocket_db_pools::{Connection, Database};

// =========================================

#[derive(Database)] // A Rust macro that operates on the DsuToolsDB struct.
#[database("dsutools_db")] // Links the DsuToolsDB struct to the "dsutools_db" database mentioned in the Rocket.toml file.
struct DsuToolsDB(sqlx::SqlitePool); // A struct with one field, a SqlitePool.

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
async fn login(
    user: Json<User>,
    mut db: Connection<DsuToolsDB>,
) -> Result<Json<LoginResponse>, Status> {
    // This function returns a Json<LoginResponse> on success or a Status code on failure.

    // Generate a SQL query
    let query = sqlx::query("SELECT * FROM users WHERE username = ? AND password_hash = ?")
        .bind(&user.username)
        .bind(&user.password);

    // Perform the query to the database.
    let query_result = query.fetch_one(&mut **db).await; // Asyncronous Rust keyword. This function will wait for the database to respond.

    // query_result is a Result type, which can be Ok(<some value>) or Err(<some value>).
    // If it is an error, ...
    if let Err(_) = query_result {
        // If the query fails, return a HTTP 401 Unauthorized status code.
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

// 'rocket::main' is another macro that tells Rocket that this is our main function.
// While compiling, Rocket will modify this function using witchcraft.
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // This code starts up Rocket. It tells Rocket about our 'index()' function
    // we've defined above, and where to mount it.
    // Finally, it runs the server until the server is stopped.
    let _rocket = rocket::build()
        .attach(DsuToolsDB::init()) // Use this database.
        .mount("/", routes![login, failed_login])
        .mount("/", FileServer::from("../dsutools-frontend/dist/"))
        .launch()
        .await?;

    // At this point, the server has stopped.

    // In Rust, we have a Result type that is either Ok or an Error.
    // Here we are returning the Ok value.
    return Ok(());
}

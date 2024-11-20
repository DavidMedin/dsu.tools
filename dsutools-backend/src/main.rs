#[macro_use]
extern crate rocket;

// Importing libraries ====================
// Rocket (our web framework)
use rocket::fs::FileServer;
use rocket::http::{CookieJar, Status};
use rocket::serde::{json::Json, Deserialize};

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

// Random Numbers
use rand::{distributions::Alphanumeric, Rng};
// =========================================

// Session tokens are 128 characters long with alphanumberic (a-Z, 0-9) characters.
fn new_session_token() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    return s;
}

async fn get_user_id(db: &mut Connection<DsuToolsDB>, username: &String) -> Option<i64> {
    // Generate a SQL query
    let query = sqlx::query("SELECT * FROM Users WHERE username = ?").bind(username);

    // Perform the query to the database.
    let query_result = query.fetch_one(&mut ***db).await;

    match query_result {
        Ok(result_row) => {
            let id: i64 = result_row.get("id");
            return Some(id);
        }
        Err(e) => {
            eprintln!("Failed to get user from username : {}", e);
            return None;
        }
    }
}

async fn is_token_authenticated(
    db: &mut Connection<DsuToolsDB>,
    user_id: i64,
    token: &String,
) -> bool {
    // Generate a SQL query
    let query = sqlx::query("SELECT * FROM SessionTokens WHERE user_id = ? AND token = ?")
        .bind(user_id)
        .bind(token);

    // Ask the database the question
    let query_result = query.fetch_one(&mut ***db).await;

    return query_result.is_ok();
}

async fn register_session_token(
    db: &mut Connection<DsuToolsDB>,
    user_id: i64,
    token: &String,
) -> Result<(), ()> {
    // Generate a SQL query
    let query = sqlx::query("INSERT INTO SessionTokens (user_id, token) VALUES (?, ?)")
        .bind(user_id)
        .bind(token);

    // Ask the database the question
    let query_result = query.execute(&mut ***db).await;

    if let Err(e) = query_result {
        eprintln!("Failed to register token : {}", e);
        return Err(());
    }
    return Ok(());
}

async fn delete_session_token(
    db: &mut Connection<DsuToolsDB>,
    user_id: i64,
    user_token: &String,
) -> Result<(), ()> {
    // Generate a SQL query
    let query = sqlx::query("DELETE FROM SessionTokens WHERE user_id = ? AND token = ?")
        .bind(user_id)
        .bind(user_token);

    // Ask the database the question
    let query_result = query.execute(&mut ***db).await;

    if let Err(e) = query_result {
        eprintln!("Failed to delete token : {}", e);
        return Err(());
    }
    return Ok(());
}

#[derive(Database)] // A Rust macro that operates on the DsuToolsDB struct.
#[database("dsutools_db")] // Links the DsuToolsDB struct to the "dsutools_db" database mentioned in the Rocket.toml file.
struct DsuToolsDB(sqlx::SqlitePool); // A struct with one field, a SqlitePool.

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct SessionUser {
    username: String,
    token: String,
}

// v------ This is a Function Macro in Rust. It is some code that the Rocket library
//         Defines to make it easier to make a route.
#[post("/login", format = "application/json", data = "<user>")]
async fn login(
    user: Json<User>,
    mut db: Connection<DsuToolsDB>,
    cookies: &CookieJar<'_>,
) -> Status {
    // This function returns a Json<LoginResponse> on success or a Status code on failure.

    // 1. Test to see if this user exists and has this password.
    // Generate a SQL query
    let query = sqlx::query("SELECT * FROM users WHERE username = ? AND password_hash = ?")
        .bind(&user.username)
        .bind(&user.password);

    // Perform the query to the database.
    let query_result = query.fetch_one(&mut **db).await; // Asyncronous Rust keyword. This function will wait for the database to respond.

    // query_result is a Result type, which can be Ok(<some value>) or Err(<some value>).
    let user_row = match query_result {
        Ok(row) => row, // the variable 'user_row' will be set to the value of 'row' if the query was successful.
        Err(_) => {
            // If the query fails, return a HTTP 401 Unauthorized status code.
            return Status::Unauthorized;
        }
    };

    // 2. Get the user's ID from the result.
    let user_id: i64 = user_row.get("id");

    // 3. Generate a new session token.
    let mut token: String = new_session_token();

    // 4. Test to see if this session_token exists in the database.
    //    Regenerate the token if it already exists.
    while is_token_authenticated(&mut db, user_id, &token).await {
        // If the token already exists, generate a new one.
        token = new_session_token();
    }

    // 5. Register the token in the database.
    register_session_token(&mut db, user_id, &token)
        .await
        .unwrap(); // TODO: don't unwrap. Handle this error.

    //6. Add the token to the session cookie.
    //TODO: Make this secure.
    cookies.add(("session", token.clone()));

    // TODO: Remove token resonse.
    return Status::Ok;
}

// If the login request doesn't have the corrent user information required,...
#[post("/login", rank = 2)]
fn failed_login() -> Status {
    // TODO: pick better error codes.
    return Status::Unauthorized;
}

#[post("/logout", format = "application/json", data = "<user>")]
async fn logout(
    user: Json<SessionUser>,
    mut db: Connection<DsuToolsDB>,
    cookies: &CookieJar<'_>,
) -> Status {

    // 1. Test to see if this user exists.
    // Generate a SQL query
    let user_query = sqlx::query("SELECT * FROM users WHERE username = ?")
    .bind(&user.username);

    let user_query_result = user_query.fetch_one(&mut **db).await; // Asyncronous Rust keyword. This function will wait for the database to respond.
    
    let user_row = match user_query_result {
        Ok(row) => row, // the variable 'user_row' will be set to the value of 'row' if the query was successful.
        Err(_) => {
            // If the query fails, return a HTTP 401 Unauthorized status code.
            return Status::Unauthorized;
        }
    };

    let user_id = user_row.get("id");

    // 2. Test to see if this session_token exists in the database.
    let session_query = sqlx::query("SELECT * FROM SessionTokens WHERE user_id = ? AND token = ?")
        .bind(&user_id)
        .bind(&user.token);

    let session_query_result = session_query.fetch_one(&mut **db).await; // Asyncronous Rust keyword. This function will wait for the database to respond.
    
    if let Err(_) = session_query_result {
        // If the query fails, return a HTTP 401 Unauthorized status code.
        return Status::Unauthorized;
    }

    // 3. Register the token from the database.
    delete_session_token(&mut db, user_id, &user.token)
        .await
        .unwrap();

    // 4. Delete the token from the session cookie.
    cookies.remove(("session", user.token.clone()));

    return Status::Ok;
}

#[post("/logout", rank = 2)]
fn failed_logout() -> Status {
    // TODO: pick better error codes.
    return Status::Unauthorized;
}

#[post("/register", format = "application/json", data = "<user>")]
async fn register(
    user: Json<User>,
    mut db: Connection<DsuToolsDB>,
    cookies: &CookieJar<'_>,
) -> Status {
    // 1. Test to see if this user already exists.
    if let Some(_) = get_user_id(&mut db, &user.username).await {
        return Status::ImATeapot;
    }

    // 2. If the uesr doesn't exist, create a new user.
    let query = sqlx::query("INSERT INTO Users (username, password_hash) VALUES (?, ?)")
        .bind(&user.username)
        .bind(&user.password);
    let query_result = query.fetch_one(&mut **db).await;
    let user_id = match query_result {
        Ok(row) => row.get("id"),
        Err(e) => {
            eprintln!("Failed to register user : {}", e);
            return Status::ImATeapot;
        }
    };

    // 3. Generate a new session token.
    let mut token: String = new_session_token();

    // 4. Test to see if this session_token exists in the database.
    //    Regenerate the token if it already exists.
    while is_token_authenticated(&mut db, user_id, &token).await {
        // If the token already exists, generate a new one.
        token = new_session_token();
    }

    // 5. Register the token in the database.
    register_session_token(&mut db, user_id, &token)
        .await
        .unwrap(); // TODO: don't unwrap. Handle this error.
                   // 6. Log the user in

    cookies.add(("session", token.clone()));
    return Status::Ok;
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
        .mount("/", routes![login, failed_login, register, logout, failed_logout])
        .mount("/", FileServer::from("../dsutools-frontend/dist/"))
        .launch()
        .await?;

    // At this point, the server has stopped.

    // In Rust, we have a Result type that is either Ok or an Error.
    // Here we are returning the Ok value.
    return Ok(());
}
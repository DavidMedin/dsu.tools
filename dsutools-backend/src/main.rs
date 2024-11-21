#[macro_use]
extern crate rocket;

// Importing libraries ====================
// Rocket (our web framework)
use rocket::fs::FileServer;
use rocket::http::{CookieJar, Status};
use rocket::serde::{json::Json, Deserialize};

// Database (also Rocket)
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

// Random Numbers
use rand::{distributions::Alphanumeric, Rng};

// Logging
use log::{debug, error, info, trace, warn};
// =========================================

// Session tokens are 128 characters long with alphanumberic (a-Z, 0-9) characters.
fn new_session_token() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();

    // I'm doing some 'complex' logging here.
    // There are a few logging levels including debug,error, info, trace, and warn (may be others).
    // They are sorted by severity and verbosity. Trace is more verbose than debug; very super verbose.
    // the macro `trace!` comes from the `log` library.
    // The first argument here is making a strucured log: print key-value pairs in the log.
    // `token:? = s` says "make a key named 'token' and its value is from the variable 's'. ':?' says use the debug formatter for 'token'."
    info!(token:? = s; "Generated new Session Token");
    return s;
}

async fn get_user_id(
    db: &mut Connection<DsuToolsDB>,
    username: &String,
) -> Result<Option<i64>, sqlx::Error> {
    // Generate a SQL query
    let query = sqlx::query("SELECT * FROM Users WHERE username = ?").bind(username);

    // Perform the query to the database.
    let query_result = query.fetch_one(&mut ***db).await;

    match query_result {
        Ok(result_row) => {
            let id: i64 = result_row.get("id");
            return Ok(Some(id));
        }
        Err(e) => {
            if let sqlx::Error::RowNotFound = e {
                // The user doesn't exist; gracefully return.
                return Ok(None);
            } else {
                // Something else is wrong!
                error!("Failed to get query DB for User ID with Username : {}", e);
                return Err(e);
            }
        }
    }
}

async fn is_token_authenticated(
    db: &mut Connection<DsuToolsDB>,
    user_id: i64,
    token: &String,
) -> Result<bool, sqlx::Error> {
    // Generate a SQL query
    let query = sqlx::query("SELECT * FROM SessionTokens WHERE user_id = ? AND token = ?")
        .bind(user_id)
        .bind(token);

    // Ask the database the question
    let query_result = query.fetch_one(&mut ***db).await;

    match query_result {
        Ok(_) => return Ok(true),
        Err(e) => {
            if let sqlx::Error::RowNotFound = e {
                return Ok(false);
            } else {
                error!(
                    "Failed to get query DB for Session Token with User ID and Token : {}",
                    e
                );
                return Err(e);
            }
        }
    }
}

async fn register_session_token(
    db: &mut Connection<DsuToolsDB>,
    user_id: i64,
    token: &String,
) -> Result<(), sqlx::Error> {
    // Generate a SQL query
    let query = sqlx::query("INSERT INTO SessionTokens (user_id, token) VALUES (?, ?)")
        .bind(user_id)
        .bind(token);

    // Ask the database the question
    let query_result = query.execute(&mut ***db).await;
    query_result?; // Return if query_result is an Err(...)

    return Ok(());
}

async fn delete_session_token(
    db: &mut Connection<DsuToolsDB>,
    user_id: i64,
    user_token: &String,
) -> Result<(), sqlx::Error> {
    // Generate a SQL query
    let query = sqlx::query("DELETE FROM SessionTokens WHERE user_id = ? AND token = ?")
        .bind(user_id)
        .bind(user_token);

    // Ask the database the question
    let query_result = query.execute(&mut ***db).await;

    if let Err(e) = query_result {
        error!("Failed to delete token : {}", e);
        return Err(e);
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
    while match is_token_authenticated(&mut db, user_id, &token).await {
        Ok(found) => found,
        Err(e) => {
            error!("Failed to test if token is authenticated : {}", e);
            return Status::InternalServerError;
        }
    } {
        // If the token already exists, generate a new one.
        token = new_session_token();
    }

    // 5. Register the token in the database.
    if let Err(e) = register_session_token(&mut db, user_id, &token).await {
        error!("Failed to register Session Token into DB : {}", e);
        return Status::InternalServerError; // Database state is likely broken now.
                                            // TODO: Maybe make sqlite sessions?
    }

    //6. Add the token to the session cookie.
    //TODO: Make this secure.
    cookies.add(("session", token.clone()));

    return Status::Ok;
}

// If the login request doesn't have the corrent user information required,...
#[post("/login", rank = 2)]
fn bad_login() -> Status {
    return Status::BadRequest;
}

#[post("/logout", format = "application/json", data = "<user>")]
async fn logout(
    user: Json<SessionUser>,
    mut db: Connection<DsuToolsDB>,
    cookies: &CookieJar<'_>,
) -> Status {
    // 1. Get User ID from Username.
    let user_id = match get_user_id(&mut db, &user.username).await {
        Ok(maybe) => match maybe {
            Some(id) => id,
            None => return Status::Unauthorized,
        },
        Err(e) => {
            error!("Failed to query DB for User ID from Username : {}", e);
            return Status::InternalServerError;
        }
    };

    // 2. Test is this user is authenticated with this session token
    match is_token_authenticated(&mut db, user_id, &user.token).await {
        Ok(false) => return Status::Unauthorized,
        Ok(true) => {}
        Err(e) => {
            error!(
                "Failed to test if User is authenticated with Session Token : {}",
                e
            );
        }
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
fn bad_logout() -> Status {
    return Status::BadRequest;
}

#[post("/register", format = "application/json", data = "<user>")]
async fn register(
    user: Json<User>,
    mut db: Connection<DsuToolsDB>,
    cookies: &CookieJar<'_>,
) -> Status {
    // 1. Test to see if this user already exists.
    match get_user_id(&mut db, &user.username).await {
        Ok(Some(_)) => {
            return Status::Conflict; // The user already exists.
        }
        Err(e) => {
            error!("Failed to test if user already exists : {}", e);
            return Status::InternalServerError;
        }
        _ => {} // The 'else' clause. This is executed when the user is not in the database.
    }

    // 2. If the user doesn't exist, create a new user.
    let query = sqlx::query("INSERT INTO Users (username, password_hash) VALUES (?, ?)")
        .bind(&user.username)
        .bind(&user.password);
    let query_result = query.execute(&mut **db).await;
    let user_id = match query_result {
        Ok(row) => row.last_insert_rowid(),
        Err(e) => {
            error!("Failed to register user : {}", e);
            return Status::InternalServerError;
        }
    };

    // 3. Generate a new session token.
    let mut token: String = new_session_token();

    // 4. Test to see if this session_token exists in the database.
    //    Regenerate the token if it already exists.
    while match is_token_authenticated(&mut db, user_id, &token).await {
        Ok(found) => found,
        Err(e) => {
            error!("Failed to test if token is authenticated : {}", e);
            return Status::InternalServerError;
        }
    } {
        // If the token already exists, generate a new one.
        token = new_session_token();
    }

    // 5. Register the token in the database.
    if let Err(e) = register_session_token(&mut db, user_id, &token).await {
        error!("Failed to register Session Token into DB : {}", e);
        return Status::InternalServerError; // Database state is likely broken now. Maybe make sqlite sessions?
    }

    cookies.add(("session", token.clone()));
    return Status::Created;
}

#[post("/register", rank = 2)]
async fn bad_register() -> Status {
    return Status::BadRequest;
}

#[delete("/user", format = "application/json", data = "<user>")]
async fn delete_user(
    user: Json<SessionUser>,
    mut db: Connection<DsuToolsDB>,
    cookies: &CookieJar<'_>,
) -> Status {
    // 1. Test to see if this user exists.
    let user_id = match get_user_id(&mut db, &user.username).await {
        Ok(maybe) => match maybe {
            Some(id) => id,
            None => return Status::BadRequest,
        },
        Err(e) => {
            error!("Failed to test if a user exists : {}", e);
            return Status::InternalServerError;
        }
    };

    // 2. Test if the token is authentication.
    match is_token_authenticated(&mut db, user_id, &user.token).await {
        Ok(false) => return Status::Forbidden,
        Ok(true) => {}
        Err(e) => {
            error!("Failed to test if session token is authenticated : {}", e);
            return Status::InternalServerError;
        }
    }

    // 3. If the user exists, delete the user and the session token.
    let query_delete = sqlx::query(
        "DELETE FROM SessionTokens WHERE username = ?; DELETE FROM Users WHERE username = ?",
    )
    .bind(&user.username)
    .bind(&user.username);
    let query_result = query_delete.fetch_one(&mut **db).await;

    if let Err(e) = query_result {
        error!("Failed to delete a user : {}", e);
        return Status::InternalServerError;
    }

    cookies.remove(("session", user.token.clone()));
    return Status::Ok;
}

#[delete("/user", rank = 2)]
fn bad_delete_user() -> Status {
    return Status::BadRequest;
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
        .mount(
            "/",
            routes![
                login,
                bad_login,
                register,
                bad_register,
                logout,
                bad_logout,
                delete_user,
                bad_delete_user
            ],
        )
        .mount("/", FileServer::from("../dsutools-frontend/dist/"))
        .launch()
        .await?;
    // At this point, the server has stopped.

    // In Rust, we have a Result type that is either Ok or an Error.
    // Here we are returning the Ok value.
    return Ok(());
}

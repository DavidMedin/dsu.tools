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

// Password Hashing : "don't roll your own"
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use serde::Serialize;
// Logging
use tracing::{error, instrument, trace, warn};
// =========================================

// Session tokens are 128 characters long with alphanumberic (a-Z, 0-9) characters.
#[instrument] // Tells the 'tracing' library to wrap this function in some function calls that allow logging.
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
    trace!(token = s, "Generated new Session Token");
    return s;
}

// Verify if an password matches an Argon2 password hash. Our database contains Argon2 hashes.
#[instrument]
fn verify_password(
    password: &String,
    db_password_hash: &String,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(db_password_hash)?;
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
    trace!(
        hash_result = ?result,
        "Checked if password matches password hash."
    );
    return result;
}

#[instrument]
fn hash_password(password: &String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    return Ok(password_hash);
}

#[instrument(skip(db))]
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

#[instrument(skip(db))]
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

#[instrument(skip(db))]
async fn get_user_token(
    db: &mut Connection<DsuToolsDB>,
    user_id: i64,
) -> Result<Option<String>, sqlx::Error> {
    // Generate s SQL query
    let query = sqlx::query("SELECT * FROM SessionTokens WHERE user_id = ?").bind(user_id);

    // Send the query to the database.
    let query_result = query.fetch_one(&mut ***db).await;

    match query_result {
        Ok(result_row) => {
            let token: String = result_row.get("token");
            return Ok(Some(token));
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

#[instrument(skip(db))]
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

#[instrument(skip(db))]
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

#[instrument(skip(db))]
async fn get_flashcard_deck_id(
    db: &mut Connection<DsuToolsDB>,
    user_id: i64,
    flashcard_deck_name: &String,
) -> Result<Option<i64>, sqlx::Error> {
    // Generate a SQL query.
    let query = sqlx::query("SELECT * FROM FlashcardDecks WHERE deck_name = ? AND user_id = ?")
        .bind(flashcard_deck_name)
        .bind(user_id);

    // Perform the query to the database.
    let query_result = query.fetch_one(&mut ***db).await;

    match query_result {
        Ok(result_row) => {
            let id: i64 = result_row.get("id");
            return Ok(Some(id));
        }
        Err(e) => {
            if let sqlx::Error::RowNotFound = e {
                // The flashcard deck doesn't exist; gracefully return.
                return Ok(None);
            } else {
                // Something else is wrong!
                return Err(e);
            }
        }
    }
}

#[instrument(skip(db))]
async fn register_flashcard_deck(
    db: &mut Connection<DsuToolsDB>,
    user_id: i64,
    deck_name: &String,
    deck_description: &String,
) -> Result<(), sqlx::Error> {
    // Generate a SQL query
    let query = sqlx::query(
        "INSERT INTO FlashcardDecks (user_id, deck_name, deck_description) VALUES (?, ?, ?)",
    )
    .bind(user_id)
    .bind(deck_name)
    .bind(deck_description);

    // Send the query to the database.
    let query_result = query.execute(&mut ***db).await;
    query_result?; // Return if query_result is an Err(...)

    return Ok(());
}

#[instrument(skip(db))]
async fn add_flashcards_to_deck(
    db: &mut Connection<DsuToolsDB>,
    deck_id: i64,
    flashcards: &Vec<FlashcardCreateDesciptor>,
) -> Result<Vec<i64>, sqlx::Error> {
    let mut primary_keys: Vec<i64> = vec![]; // Create an empty vector of integers.

    for flashcard in flashcards {
        let query = sqlx::query(
            "INSERT INTO Flashcards (flashcard_deck_id, front, back) VALUES (?, ?, ?)",
        )
        .bind(&deck_id)
        .bind(&flashcard.flashcard_front)
        .bind(&flashcard.flashcard_back);

        let row = query.execute(&mut ***db).await?;
        let primary_key = row.last_insert_rowid();
        primary_keys.push(primary_key);
    }

    return Ok(primary_keys);
}

#[instrument(skip(db))]
async fn get_flashcards_in_deck(
    db: &mut Connection<DsuToolsDB>,
    deck_id: i64,
) -> Result<Vec<Flashcard>, sqlx::Error> {
    let card_query =
        sqlx::query("SELECT * FROM Flashcards WHERE flashcard_deck_id = ?").bind(&deck_id);
    let rows = card_query.fetch_all(&mut ***db).await?;

    let flashies: Vec<Flashcard> = rows
        .iter() // Get an iterator of the rows.
        .map(|row| Flashcard {
            // Apply this function to every iteration of the iterator.
            id: row.get("id"),
            flashcard_front: row.get("front"),
            flashcard_back: row.get("back"),
        })
        .collect(); // Convert iterator into the desiered type (Vec<Flashcard> in this case).

    return Ok(flashies);
}

// Very high level function that tests if the user exists and is authenticated.
// Also returns the user id, because it is useful.
async fn test_user(
    db: &mut Connection<DsuToolsDB>,
    cookies: &CookieJar<'_>,
    username: &String,
) -> Result<i64, Status> {
    // 1. Test the user exists.
    let user_id = match get_user_id(db, username).await {
        Ok(id) => match id {
            Some(i) => i,
            None => return Err(Status::BadRequest),
        },
        Err(e) => {
            error!(username = username, ?e);
            return Err(Status::Unauthorized);
        }
    };

    // 2. Test the user is authenticated.
    let session_token = match cookies.get("session") {
        Some(token) => token.value().to_string(),
        None => {
            return Err(Status::Unauthorized);
        }
    };
    match is_token_authenticated(db, user_id, &session_token).await {
        Ok(is_authed) => {
            if false == is_authed {
                return Err(Status::Unauthorized);
            }
        }
        Err(e) => {
            error!("Failed to test if session token is valid. : {}", e);
            return Err(Status::InternalServerError);
        }
    }
    return Ok(user_id);
}

#[derive(Database)] // A Rust macro that operates on the DsuToolsDB struct.
#[database("dsutools_db")] // Links the DsuToolsDB struct to the "dsutools_db" database mentioned in the Rocket.toml file.
struct DsuToolsDB(sqlx::SqlitePool); // A struct with one field, a SqlitePool.

#[derive(Deserialize, Debug)]
struct NewUser {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct SessionUser {
    username: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct CreateNewFlashcardDeck {
    username: String,
    flashcard_deck: FlashcardDeck,
}

#[derive(Deserialize, Serialize, Debug)]
struct FlashcardDeck {
    name: String,
    description: String,
}

// v------ This is a Function Macro in Rust. It is some code that the Rocket library
//         Defines to make it easier to make a route.
#[instrument(skip(db, cookies))]
#[post("/login", format = "application/json", data = "<user>")]
async fn login(
    user: Json<NewUser>,
    mut db: Connection<DsuToolsDB>,
    cookies: &CookieJar<'_>,
) -> Status {
    // This function returns a Json<LoginResponse> on success or a Status code on failure.

    // 1. Test to see if this user exists and has this password.
    // Generate a SQL query
    let query = sqlx::query("SELECT * FROM users WHERE username = ?").bind(&user.username);

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

    // 1.2. Test if the password given matches the password hash in the database.
    if let Err(e) = verify_password(&user.password, &user_row.get("password_hash")) {
        if e == argon2::password_hash::Error::Password {
            return Status::Unauthorized;
        }
        return Status::InternalServerError;
    }

    // 2. Get the user's ID from the result.
    let user_id: i64 = user_row.get("id");

    // 3. Check if the user is already logged in.
    match get_user_token(&mut db, user_id).await {
        Ok(Some(_)) => {
            // If the user is already logged in, return a HTTP 409 Conflict status code.
            return Status::Conflict;
        }
        Ok(None) => {}
        Err(e) => {
            error!(
                "Failed to test if a session token under that user id exists: {}",
                e
            );
            return Status::InternalServerError;
        }
    }

    // 4. Generate a new session token.
    let mut token: String = new_session_token();

    // 5. Test to see if this session_token exists in the database.
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

    // 6. Register the token in the database.
    if let Err(e) = register_session_token(&mut db, user_id, &token).await {
        error!("Failed to register Session Token into DB : {}", e);
        return Status::InternalServerError; // Database state is likely broken now.
                                            // TODO: Maybe make sqlite sessions?
    }

    //7. Add the token to the session cookie.
    //TODO: Make this secure.
    cookies.add(("session", token.clone()));

    return Status::Ok;
}

// If the login request doesn't have the corrent user information required,...
#[instrument]
#[post("/login", rank = 2)]
fn bad_login() -> Status {
    return Status::BadRequest;
}

#[instrument(skip(db, cookies))]
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

    // 2. Verify the user is logged in aka if session token under that user exists.
    let token = match get_user_token(&mut db, user_id).await {
        Ok(maybe) => match maybe {
            Some(token) => token,
            None => return Status::BadRequest,
        },
        Err(e) => {
            error!(
                "Failed to test if a session token under that user id exists: {}",
                e
            );
            return Status::InternalServerError;
        }
    };

    // 3. Test is this user is authenticated with this session token
    match is_token_authenticated(&mut db, user_id, &token).await {
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
    delete_session_token(&mut db, user_id, &token)
        .await
        .unwrap();

    // 4. Delete the token from the session cookie.
    cookies.remove(("session", token));

    return Status::Ok;
}

#[instrument]
#[post("/logout", rank = 2)]
fn bad_logout() -> Status {
    return Status::BadRequest;
}

#[instrument(skip(db, cookies))]
#[post("/register", format = "application/json", data = "<user>")]
async fn register(
    user: Json<NewUser>,
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

    //2.1. Create a hash of the password
    let password_hash: String = match hash_password(&user.password) {
        Ok(hash) => hash,
        Err(_) => {
            return Status::InternalServerError;
        }
    };

    //2.2. Insert the user into the database.
    let query = sqlx::query("INSERT INTO Users (username, password_hash) VALUES (?, ?)")
        .bind(&user.username)
        .bind(&password_hash);
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

#[instrument]
#[post("/register", rank = 2)]
async fn bad_register() -> Status {
    return Status::BadRequest;
}

#[instrument(skip(db, cookies))]
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

    // 2. Verify the user is logged in aka if session token under that user exists.
    let token = match get_user_token(&mut db, user_id).await {
        Ok(maybe) => match maybe {
            Some(token) => token,
            None => return Status::BadRequest,
        },
        Err(e) => {
            error!(
                "Failed to test if a session token under that user id exists: {}",
                e
            );
            return Status::InternalServerError;
        }
    };

    // 3. Test if the token is authentication.
    match is_token_authenticated(&mut db, user_id, &token).await {
        Ok(false) => return Status::Forbidden,
        Ok(true) => {}
        Err(e) => {
            error!("Failed to test if session token is authenticated : {}", e);
            return Status::InternalServerError;
        }
    }

    // 4. If the user exists, delete the user and the session token.
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

    cookies.remove(("session", token.clone()));
    return Status::Ok;
}

#[instrument]
#[delete("/user", rank = 2)]
fn bad_delete_user() -> Status {
    return Status::BadRequest;
}

#[instrument(skip(db))]
#[post(
    "/create-flashcard-deck",
    format = "application/json",
    data = "<flashcard_deck_request>"
)]
async fn create_flashcard_deck(
    flashcard_deck_request: Json<CreateNewFlashcardDeck>,
    mut db: Connection<DsuToolsDB>,
) -> Status {
    let username = &flashcard_deck_request.username;
    let flashcard_deck_name = &flashcard_deck_request.flashcard_deck.name;
    let flashcard_deck_description = &flashcard_deck_request.flashcard_deck.description;

    // Verify the user exists.
    let user_id = match get_user_id(&mut db, username).await {
        Ok(maybe) => match maybe {
            Some(id) => id,
            None => return Status::BadRequest,
        },
        Err(e) => {
            error!("Failed to test if a user exists : {}", e);
            return Status::InternalServerError;
        }
    };

    // Verify the user is logged in aka if session token under that user exists.
    match get_user_token(&mut db, user_id).await {
        Ok(maybe) => match maybe {
            Some(token) => token,
            None => return Status::BadRequest,
        },
        Err(e) => {
            error!(
                "Failed to test if a session token under that user id exists: {}",
                e
            );
            return Status::InternalServerError;
        }
    };

    // Test to see if the flashcard deck name already exists.
    match get_flashcard_deck_id(&mut db, user_id, &flashcard_deck_name).await {
        Ok(Some(_)) => {
            return Status::Conflict; // the flashcard deck name already exists
        }
        Err(e) => {
            error!(
                "Failed to test if the flashcard deck name already exists : {}",
                e
            );
            return Status::InternalServerError;
        }
        _ => {} // The 'else' clause. This is executed when the flashcard deck name is not in the database
    }

    if let Err(e) = register_flashcard_deck(
        &mut db,
        user_id,
        &flashcard_deck_name,
        &flashcard_deck_description,
    )
    .await
    {
        error!("Failed to register Flashcard Deck into DB: {}", e);
        return Status::InternalServerError;
    }

    return Status::Created;
}

#[instrument]
#[post("/create-flashcard-deck", rank = 2)]
async fn bad_create_flashcard_deck() -> Status {
    return Status::BadRequest;
}

#[instrument(skip(db, cookies))]
#[get("/get-flashcard-decks?<username>")]
async fn get_flashcard_decks(
    username: &str,
    mut db: Connection<DsuToolsDB>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Vec<FlashcardDeck>>, Status> {
    // Verify the user exists.
    let user_id = match get_user_id(&mut db, &username.to_string()).await {
        Ok(maybe) => match maybe {
            Some(id) => id,
            None => return Err(Status::Unauthorized),
        },
        Err(e) => {
            error!("Failed to query DB for User ID from Username: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    // Get the session token associated with the user ID.
    match cookies.get("session") {
        Some(cookie) => cookie.value().to_string(),
        None => return Err(Status::Unauthorized),
    };

    // Generate a SQL query for flashcard deck names.
    let query = sqlx::query("SELECT * FROM FlashcardDecks WHERE user_id = ?").bind(user_id);

    // Perform the query to the database.
    let query_result = query.fetch_all(&mut **db).await;

    // return the flashcard decks
    match query_result {
        Ok(result) => {
            let mut flashcard_decks: Vec<FlashcardDeck> = Vec::new();
            for row in result {
                let deck_name: String = row.get("deck_name");
                let deck_description: String = row.get("deck_description");
                flashcard_decks.push(FlashcardDeck {
                    name: deck_name,
                    description: deck_description,
                });
            }
            return Ok(Json(flashcard_decks));
        }
        Err(e) => {
            error!("Failed to query DB for Flashcard Decks: {}", e);
            return Err(Status::InternalServerError);
        }
    }
}

#[instrument]
#[get("/get-flashcard-decks", rank = 2)]
async fn bad_get_flashcard_decks() -> Status {
    return Status::BadRequest;
}

// Used for describing the creation of a flashcard.
#[derive(Deserialize, Debug)]
struct FlashcardCreateDesciptor {
    flashcard_front: String,
    flashcard_back: String,
}

#[derive(Deserialize, Debug)]
struct CreateFlashcardsBody {
    username: String,
    flashcard_deck_name: String,
    flashcards: Vec<FlashcardCreateDesciptor>,
}

#[derive(Serialize, Debug)]
struct Flashcard {
    id: i64,
    flashcard_front: String,
    flashcard_back: String,
}

#[instrument(skip(db, cookies))]
#[post("/create-flashcards", format = "application/json", data = "<body>")]
async fn create_flashcards(
    body: Json<CreateFlashcardsBody>,
    mut db: Connection<DsuToolsDB>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Vec<i64>>, Status> {
    // 1. Test the user exists and is authenticated.
    let user_id: i64 = test_user(&mut db, cookies, &body.username).await?;

    // 2. Test that the requested flashcard deck exists and is this user's
    let flashcard_deck_id =
        match get_flashcard_deck_id(&mut db, user_id, &body.flashcard_deck_name).await {
            Ok(Some(id)) => id,
            Ok(None) => return Err(Status::BadRequest),
            Err(e) => {
                error!(
                    username = body.username,
                    flashcard_deck_name = body.flashcard_deck_name,
                    ?e
                );
                return Err(Status::BadRequest);
            }
        };

    // 3. Add flashcards into database
    let card_ids: Vec<i64> =
        match add_flashcards_to_deck(&mut db, flashcard_deck_id, &body.flashcards).await {
            Ok(card_ids) => card_ids,
            Err(e) => {
                error!(
                    "Failed to add flashcards to deck. DB is likely broken now. :) : {}",
                    e
                );
                return Err(Status::InternalServerError);
            }
        };
    return Ok(Json(card_ids));
}

#[post("/create-flashcards", rank = 2)]
async fn bad_create_flashcards() -> Status {
    return Status::BadRequest;
}

#[get("/flashcard-deck?<username>&<flashcard_deck_name>")]
async fn get_flashcard_deck(
    username: String,
    flashcard_deck_name: String,
    mut db: Connection<DsuToolsDB>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Vec<Flashcard>>, Status> {
    // 1. Test the user is authenticated.
    let user_id = test_user(&mut db, cookies, &username).await?;

    // 2. Get flashcards in deck.
    let deck_id = match get_flashcard_deck_id(&mut db, user_id, &flashcard_deck_name).await {
        Ok(Some(id)) => id,
        Ok(None) => {
            return Err(Status::BadRequest);
        }
        Err(e) => {
            error!("Failed to get flashcard deck id : {}", e);
            return Err(Status::InternalServerError);
        }
    };

    let flashies: Vec<Flashcard> = match get_flashcards_in_deck(&mut db, deck_id).await {
        Ok(flashies) => flashies,
        Err(e) => {
            error!("Failed to get flashcards in a deck : {} ", e);
            return Err(Status::InternalServerError);
        }
    };

    return Ok(Json(flashies));
}

#[get("/flashcard-deck", rank = 2)]
fn bad_get_flashcard_deck() -> Status {
    return Status::BadRequest;
}

// 'rocket::main' is another macro that tells Rocket that this is our main function.
// While compiling, Rocket will modify this function using witchcraft.
#[instrument]
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Describes to the 'tracing' library how and what to log.
    let log_subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_file(false)
        .with_line_number(true)
        .with_env_filter("dsutools_backend=trace")
        .finish();
    tracing::subscriber::set_global_default(log_subscriber).unwrap();

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
                bad_delete_user,
                create_flashcard_deck,
                bad_create_flashcard_deck,
                get_flashcard_decks,
                bad_get_flashcard_decks,
                create_flashcards,
                bad_create_flashcards,
                get_flashcard_deck,
                bad_get_flashcard_deck
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

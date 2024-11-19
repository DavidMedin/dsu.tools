-- This is a SQL DDL script.
-- the Structured Query Language Data Definition Language is a sub-language of SQL.
-- It is used to create new tables and such in a SQL database.
-- This file can be run against a SQLite database to create said tables and such.
-- https://stackoverflow.com/questions/2578194

-- In a DB viewer like DBeaver you can generate a DDL script from a table.
-- Right-click on a table and select "Generate SQL" -> "DDL" to get the SQL DDL code like below.

CREATE TABLE Users (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	username TEXT NOT NULL,
	password_hash TEXT NOT NULL
);

CREATE TABLE SessionTokens (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users(id)
);

# API
This is the language that the client (the browser) and the server use to communicate.\
This language uses JSON syntax.

### POST `/login`
Request:
```json
{
    "username": "the_username",
    "token": "a big nasty token"
}
```
Response:
- `200` (Ok): A session token as a cookie.
- `500` (Internal Server Error): Something went wrong, likely with the database.

### POST `/register`
Request:
```json
{
    "username": "the_username",
    "password": "the_password"
}
```
Response:
- `201` (Created): A session token as a cookie.
- `409` (Conflict): Username already taken.
- `500` (Internal Server Error): Something went wrong, likely with the database.


### POST `/logout`
Request:
```json
{
  "username": "the_username",
  "token": "big_nasty_token"
}
```
Response:
- `200` (Ok): Nothing,
- `500` (Internal Server Error): Something went wrong, likely with the database.
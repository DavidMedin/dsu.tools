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


### DELETE `/user`
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

### POST `/create-flashcard-deck`
Request:
```json
{
  "username": "the_username",
  "flashcard_deck":
  {
    "name": "the_flashcard_deck_name",
    "description": "the_flashcard_deck_description",
  }
}
```
Response:
- `201` (Created): Nothing,
- `500` {Internal Server Error}: Something went wrong, likely with the database.

### GET `/flashcard-deck?username&flashcard_deck_name`
Response:
- `200` (Ok):
```json
[
  {
    "flashcard_id": 13434,
    "flashcard_front": "the flashcard front",
    "flashcard_back": "the flashcard back"
  },
  { ... },
  .
  .
  .
]
```

### POST `/create-flashcards`
Request:
Cookies Required:
- `session` : "big long token string"
```json
{
  "username": "the_username",
  "flashcard_deck_name": "the_flashcard_deck_name",
  "flashcards":[
    {
      "side_one_text": "the side_one_text",
      "side_two_text": "the side_two_text"
    },
    {
      ...
    },
    .
    .
    .
  ]
}
```
Response:
- `201` (Created): List of flashcards' primary keys. Ordered based on user's request.
```json
[
  125132,
  2324324,
  ...,
  .
  .
  .
]
```

### POST `/delete-flashcard`
Request
```json
{
  "username": "the_username",
  "flashcard_deck_name": "the flashcard deck name",
  "flashcard_id": 02938409235
}
```
Response:
- `200` (Ok): Nothing

# API
This is the language that the client (the browser) and the server use to communicate.\
This language uses JSON syntax.

### POST `/login`
Request:
```json
{
    "username": "the_username",
    "password": "the_password"
}
```
Response:
```json
{
    "token": "big_nasty_token"
}
```

### POST `/register`
Request:
```json
{
    "username": "the_username",
    "password": "the_password"
}
```
Response:
```json
{
    "token": "big_nasty_token"
}
```


### POST `/logout`
Request:
```json
{
  "token": "big_nasty_token",
  "username": "the_username"
}
```
Response:\
No body.

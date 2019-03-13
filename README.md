# Intro to Web stuff with Rust

## Example App:

Simple in-memory RESTful interface for car information.

### Run:

```
cargo run
```

### Test

```
cargo test
```

### Endpoints:

Create a car (example):

```json
Request:

POST /cars

{
    "make": "Toyota",
    "model": "Camry",
    "year": 2000
}

Response:

{
    "id": 1,
    "make": "Toyota",
    "model": "Camry",
    "year": 2000
}
```

Get a car

```json
Request:

GET /cars/1

Response:

{
    "id": 1,
    "make": "Toyota",
    "model": "Camry",
    "year": 2000
}
```

List cars

```json
Request:

GET /cars

Response:

[
  {
    "id": 1,
    "make": "Toyota",
    "model": "Camry",
    "year": 2000
  }
 ]
```

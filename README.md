# Intro to Web stuff with Rust

### Example App:

Simple "database" for car information.

### Endpoints:

Create a car (example):

```
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

```
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

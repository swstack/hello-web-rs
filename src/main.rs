extern crate actix_web;

pub mod models;

use actix_web:: http;
use actix_web:: server;
use actix_web:: HttpRequest;
use actix_web:: App;
use actix_web:: Responder;
use std::collections::HashMap;
use models::car::Car;

struct CarDao {
    cars: HashMap<u32, Car>

}

impl CarDao {
    fn list(&self) -> bool {
        true
    }

    fn get(&self) -> bool {
        true
    }

    fn create(&self) -> bool {
        true
    }
}

fn list_cars(req: &HttpRequest<CarDao>) -> impl Responder {
    println!("Listing cars...");
    format!("")
}

fn get_car(req: &HttpRequest<CarDao>) -> impl Responder {
    let id = 5;
    println!("Getting car {}...", id);
    format!("")
}

fn create_car(req: &HttpRequest<CarDao>) -> impl Responder {
    let id = 5;
    println!("Creating car {}...", id);
    format!("")
}

fn main() {
    server::new(|| {
        App::with_state(CarDao { cars: HashMap::new() })
        .resource("/cars", |r| r.method(http::Method::GET).f(list_cars))
        .resource("/cars/{id}", |r| r.method(http::Method::GET).f(get_car))
        .resource("/cars", |r| r.method(http::Method::POST).f(create_car))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();

//    server::new(
//        App::with_state(AppState { counter: Cell::new(0) })
//            .resource("/", |r| r.method(http::Method::GET).f(index))
//
//        || App::new()
//            .route("/cars", http::Method::GET, index)
//            .route("/cars/{id}", http::Method::GET, get_car)
//            .route("/cars", http::Method::POST, create_car)
//        )
//        .bind("127.0.0.1:8080").unwrap()
//        .run();
}
extern crate actix_web;
extern crate rand;
extern crate futures;
extern crate serde;
extern crate serde_json;

pub mod models;

use actix_web::http;
use actix_web::http::StatusCode;
use actix_web::Error;
use actix_web::server;
use actix_web::HttpRequest;
use actix_web::HttpMessage;
use actix_web::AsyncResponder;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::Result;
use std::collections::HashMap;
use futures::Future;
use futures::Stream;
use rand::Rng;

use crate::models::car::{Car, CarRequest};
use std::io::Bytes;
use std::sync::{Arc, Mutex};

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

    fn create(&mut self, car: Car) -> bool {
        self.cars.insert(car.id, car);
        true
    }
}

fn list_cars(req: &HttpRequest<Arc<Mutex<CarDao>>>) -> Result<HttpResponse> {
    println!("Listing cars...");

//    let serialized = serde_json::to_string(&deserialized).unwrap();
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .body(""))
}

fn get_car(req: &HttpRequest<Arc<Mutex<CarDao>>>) -> Result<HttpResponse> {
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen_range(0, 100);
    println!("Getting car {}...", id);

//    let serialized = serde_json::to_string(&deserialized).unwrap();
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .body(""))
}

fn create_car_async(req: &HttpRequest<Arc<Mutex<CarDao>>>) -> Box<Future<Item=HttpResponse, Error=Error>> {
//    let mut car_dao: &mut CarDao = req.state();
    let mut rng = rand::thread_rng();
    let id: u32 =  rng.gen_range(0, 100);
    println!("Creating car {}...", id);

    let car_dao = req.state().clone();

    req
    .payload()
    .concat2()
    .from_err()
    .and_then(move |body| {
        let body_string: &str = std::str::from_utf8(&body).unwrap();
        let car_request: CarRequest = serde_json::from_str(body_string).unwrap();
        let new_car: Car = Car {
            id: id,
            make: String::from(car_request.make),
            model: String::from(car_request.model),
            color: String::from(car_request.color),
            year: 40
        };

        let foo = serde_json::to_string(&new_car)

        if (foo.error) {

        } else {

        }

        let response_body = serde_json::to_string(&new_car).unwrap();
        car_dao.lock().unwrap().create(new_car);

        Ok(HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .body(response_body))
    })
    .responder()
}

fn main() {
    server::new(|| {
        App::with_state(Arc::new(Mutex::new(CarDao { cars: HashMap::new() })))
        .resource("/cars", |r| {
            r.method(http::Method::GET).f(list_cars);
            r.method(http::Method::POST).f(create_car_async);
        })
        .resource("/cars/{id}", |r| {
            r.method(http::Method::GET).f(get_car);
        })
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}

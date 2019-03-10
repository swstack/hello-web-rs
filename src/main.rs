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

use crate::models::car::Car;

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

fn list_cars(req: &HttpRequest<CarDao>) -> Result<HttpResponse> {
    println!("Listing cars...");

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .body(""))
}

fn get_car(req: &HttpRequest<CarDao>) -> Result<HttpResponse> {
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen_range(0, 100);
    println!("Getting car {}...", id);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .body(""))
}

fn create_car_async(req: &HttpRequest<CarDao>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen_range(0, 100);
    println!("Creating car {}...", id);

    req
    .payload()
    .concat2()
    .from_err()
    .and_then(|body| {
        println!("{:?}", body);
        let deserialized: Car = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
        let serialized = serde_json::to_string(&deserialized).unwrap();
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .body(serialized))
    })
    .responder()
}

fn main() {
    server::new(|| {
        App::with_state(CarDao { cars: HashMap::new() })
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

extern crate actix_web;
extern crate rand;
extern crate futures;
extern crate serde;
extern crate serde_json;

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
use std::sync::{Arc, Mutex};

pub struct CarsIterator<'a> {
    cars: Vec<&'a Car>,
    current: usize
}

impl<'a> CarsIterator<'a> {
    fn new(cars: Vec<&'a Car>) -> CarsIterator {
        return CarsIterator { cars, current: 0 }
    }
}

impl<'a> Iterator for CarsIterator<'a> {
    type Item = &'a Car;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.cars.len() {
            None
        }  else {
            let car = self.cars[self.current];
            self.current += 1;
            Some(car)
        }
    }
}

pub struct CarDao {
    cars: HashMap<usize, Car>
}

impl CarDao {
    pub fn new(cars: HashMap<usize, Car>) -> CarDao {
        return CarDao { cars }
    }

    pub fn get_all(&self) -> CarsIterator {
        let cars_vec: Vec<&Car> = self.cars.values().collect();
        let cars_iter = CarsIterator::new(cars_vec);
        return cars_iter
    }

    pub fn delete() {}

    pub fn get(&self, id: usize) -> Result<&Car, String> {
        match self.cars.get(&id) {
            Some(car) => Ok(car),
            None => Err(format!("No car with id {}", id))
        }
    }

    pub fn create(&mut self, car: Car) -> Result<Car, String> {
        match self.cars.get(&car.id) {
            Some(car) => Err(format!("Car id collision {}", &car.id)),
            None => {
                let car_clone = car.clone();
                self.cars.insert(car.id, car);
                return Ok(car_clone)
            }
        }
    }
}

fn list_cars(req: &HttpRequest<Arc<Mutex<CarDao>>>) -> Result<HttpResponse> {
    println!("Listing cars...");

    let car_dao = req.state().clone();

    let mut response_body = String::from("[");
    for c in car_dao.lock().unwrap().get_all() {
        response_body += serde_json::to_string(c).unwrap().as_str();
        response_body += ","
    }
    response_body += "]";

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .body(response_body))
}

fn get_car(req: &HttpRequest<Arc<Mutex<CarDao>>>) -> Result<HttpResponse> {
    let id = req.match_info().get("id").unwrap();
    println!("Getting car {}...", id);

    let car_dao = req.state().clone();
    let response_body: String;
    match car_dao.lock().unwrap().get(id.parse::<usize>().unwrap()) {
        Ok(car) => {
            response_body = serde_json::to_string(&car).unwrap();
        },
        Err(e) => {
            response_body = e;
        }
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .body(response_body))
}

fn create_car_async(req: &HttpRequest<Arc<Mutex<CarDao>>>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let mut rng = rand::thread_rng();
    let id: usize =  rng.gen_range(0, 100);
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
            id,
            make: String::from(car_request.make),
            model: String::from(car_request.model),
            color: String::from(car_request.color),
            year: 40
        };

        let response_body: String;
        match car_dao.lock().unwrap().create(new_car) {
            Ok(car) => {
                response_body = serde_json::to_string(&car).unwrap();
            },
            Err(e) => {
                response_body = e;
            }
        }

        Ok(HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .body(response_body))
    })
    .responder()
}

pub struct CarsApp {}

impl CarsApp {
    pub fn start() {
        let state  = Arc::new(Mutex::new(CarDao::new(HashMap::new())));
        server::new(move || {
            App::with_state(state.clone())
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
}

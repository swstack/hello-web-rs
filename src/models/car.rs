extern crate serde;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct CarRequest {
    pub make: String,
    pub model: String,
    pub color: String,
    pub year: usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Car {
    pub id: usize,
    pub make: String,
    pub model: String,
    pub color: String,
    pub year: usize
}

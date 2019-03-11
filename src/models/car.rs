extern crate serde;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct CarRequest {
    pub make: String,
    pub model: String,
    pub color: String,
    pub year: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    pub id: u32,
    pub make: String,
    pub model: String,
    pub color: String,
    pub year: u32
}

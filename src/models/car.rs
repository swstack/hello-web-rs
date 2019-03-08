extern crate serde;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    id: String,
    make: String,
    model: String,
    year: u32
}

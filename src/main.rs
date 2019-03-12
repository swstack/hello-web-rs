pub mod models;
pub mod cars;

use crate::cars::app::CarsApp;

fn main() {
    let cars_app = CarsApp::start();
}

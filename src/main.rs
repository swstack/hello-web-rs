pub mod models;
pub mod cars;

use crate::cars::app::CarsApp;

fn main() {
    CarsApp::start();
}

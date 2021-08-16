extern crate communicator;

use communicator::client;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    client::connect();
    let _red= Red;
    let _yellow = Yellow;
    let _green = TrafficLight::Green;
}
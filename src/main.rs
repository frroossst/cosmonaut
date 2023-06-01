use cosmonaut::statemachine::{RocketState, RocketAcceleration, RocketAltitude};
use cosmonaut::data_producer::DataProducer;
use cosmonaut::pid::PID;

fn main()
    {
    let mut pid = PID::new();
    pid.set_gain(0.1, 0.01, 0.01);
    pid.update(10.0, 8.0);
    }

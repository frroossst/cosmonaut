use cosmonaut::statemachine::{RocketState, RocketAcceleration, RocketAltitude};
use cosmonaut::data_producer::DataProducer;

fn main()
    {
    let mut state = RocketState::new();

    loop
        {
        let accl = RocketAcceleration::Accelerating;
        let alt = RocketAltitude::Ascending;
        state.transition(accl, alt);
        break
        }

    // let producer = DataProducer::setup("output.csv");
    DataProducer::setup(filename);
    // println!("{:?}", producer.get_data());
    }

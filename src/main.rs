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


    let mut producer = DataProducer::new();
    producer.setup("output.csv").unwrap();

    println!("{:#?}", producer.get_data());
    println!("{:#?}", producer.get_data());
    println!("{:#?}", producer.get_data());
    println!("{:#?}", producer.get_data());
    }

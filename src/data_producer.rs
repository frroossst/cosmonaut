use crate::statemachine::{RocketState, RocketAcceleration, RocketAltitude};

pub trait DataProducer 
    {
    fn setup(&self) -> Result<(), &str> ;

    fn get_data(&self) -> DataChunk;
    }

pub struct DataChunk
    {
    fram_cursor: usize,
    timestamp: u32,
    rocket_state: u8, 
    accl_x: f32,
    accl_y: f32,
    accl_z: f32,
    gyro_x: f32,
    gyro_y: f32,
    gyro_z: f32,
    relative_altitde: i32,
    absolute_pressre: f32,
    temperature: f32,
    }

impl DataChunk
    {
    fn new(fram_cursor: usize, timestamp: u32, rocket_state: u8, accl_x: f32, accl_y: f32, accl_z: f32, gyro_x: f32, gyro_y: f32, gyro_z: f32, relative_altitde: i32, absolute_pressre: f32, temperature: f32) -> DataChunk
        {
        DataChunk 
            {
            fram_cursor,
            timestamp,
            rocket_state,
            accl_x,
            accl_y,
            accl_z,
            gyro_x,
            gyro_y,
            gyro_z,
            relative_altitde,
            absolute_pressre,
            temperature,
            }
        }
    }

impl DataProducer for RocketState 
    {
    fn setup(&self) -> Result<(), &str> 
        {
        Ok(())
        }

    fn get_data(&self) -> DataChunk
        {
        DataChunk::new(fram_cursor, timestamp, rocket_state, accl_x, accl_y, accl_z, gyro_x, gyro_y, gyro_z, relative_altitde, absolute_pressre, temperature)
        }
    }

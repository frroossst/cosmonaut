use std::{vec, fmt::Debug};
use csv;

#[derive(Debug, Copy, Clone)]
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
    relative_altitde: f32,
    absolute_pressure: f32,
    temperature: f32,
    }

impl DataChunk
    {
    fn new(fram_cursor: usize, timestamp: u32, rocket_state: u8, accl_x: f32, accl_y: f32, accl_z: f32, gyro_x: f32, gyro_y: f32, gyro_z: f32, relative_altitde: f32, absolute_pressure: f32, temperature: f32) -> DataChunk
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
            absolute_pressure,
            temperature,
            }
        }
    }


#[derive(Debug)]
pub struct DataProducer
    {
    pub chunks: Vec<DataChunk>,
    cursor: usize
    }

impl DataProducer
    {  
    pub fn new() -> Self
        {
        DataProducer { chunks: vec![], cursor: 0 }
        }

    pub fn setup(&mut self, filename: &str) -> Result<(), csv::Error>
        {
        let mut csv_reader = match csv::Reader::from_path(filename)
            {
            Ok(reader) => reader,
            Err(e) => return Err(e),
            };

        let mut chunks = Vec::new();

        for record in csv_reader.records()
            {
            let record = match record
                {
                Ok(record) => record,
                Err(e) => return Err(e),
                };

            let fram_cursor = usize::from_str_radix(&record[0], 16).unwrap();
            let timestamp = record[1].parse::<u32>().unwrap();
            let rocket_state = record[2].parse::<u8>().unwrap();
            let accl_x = record[3].parse::<f32>().unwrap();
            let accl_y = record[4].parse::<f32>().unwrap();
            let accl_z = record[5].parse::<f32>().unwrap();
            let gyro_x = record[6].parse::<f32>().unwrap();
            let gyro_y = record[7].parse::<f32>().unwrap();
            let gyro_z = record[8].parse::<f32>().unwrap();
            let relative_altitde = record[9].parse::<f32>().unwrap();
            let absolute_pressure = record[10].parse::<f32>().unwrap();
            let temperature = record[11].parse::<f32>().unwrap();

            chunks.push(DataChunk::new(fram_cursor, timestamp, rocket_state, accl_x, accl_y, accl_z, gyro_x, gyro_y, gyro_z, relative_altitde, absolute_pressure, temperature));
            }

        self.chunks = chunks;

        Ok(())
        }

    pub fn get_data(&mut self) -> DataChunk
        {
        self.cursor += 1;
        self.chunks[self.cursor - 1]
        }
    }


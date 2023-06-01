#[derive(Debug)]
pub enum RocketState 
    {
    Unarmed,
    GroundIdle,
    PoweredFlight,
    UnpoweredFlight,
    Apogee,
    BallisticDescent,
    ParachuteDescent,
    LandSafe,
    Recovery
    }

impl RocketState 
    {
    pub fn new() -> RocketState 
        {
        RocketState::Unarmed
        }

    pub fn transition(&mut self, acceleration: RocketAcceleration, altitude: RocketAltitude) 
        {
        match (acceleration, altitude)
            {
            (RocketAcceleration::Accelerating, RocketAltitude::Ascending) => { *self = RocketState::PoweredFlight }
            (RocketAcceleration::Decelerating, RocketAltitude::Ascending) => { *self = RocketState::UnpoweredFlight }
            (RocketAcceleration::Decelerating, RocketAltitude::Descending) => { *self = RocketState::BallisticDescent }
            (RocketAcceleration::Coasting, RocketAltitude::Constant) => { *self = RocketState::LandSafe }
            (_, _) => { panic!("unknown state found") }
            }
        }

    pub fn arm(&mut self) 
        {
        *self = RocketState::GroundIdle;
        }
    }

pub enum RocketAcceleration 
    {
    Accelerating,
    Coasting,
    Decelerating,
    }

pub enum RocketAltitude 
    {
    Ascending,
    Constant,
    Descending,
    }


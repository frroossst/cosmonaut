use crate::data_producer::DataChunk;

pub struct PID
    {
    // Controller gains
    kp: f64,
    ki: f64,
    kd: f64,

    // Derivative low-pass filter time constant
    tau: f64,

    // Output limits
    lim_min: f64,
    lim_max: f64,
    
    // Sample time (in seconds)
    t: f64,

    // Controller memory
    integrator: f64,
    prev_error: f64,
    differentiator: f64,
    prev_measurement: f64,

    // Controller output
    output: f64,
    }

impl PID
    {
    pub fn new() -> PID
        {
        PID
            {
            kp: 0.0,
            ki: 0.0,
            kd: 0.0,
            tau: 0.0,
            lim_min: 0.0,
            lim_max: 0.0,
            t: 0.0,
            integrator: 0.0,
            prev_error: 0.0,
            differentiator: 0.0,
            prev_measurement: 0.0,
            output: 0.0,
            }
        }

    pub fn set_gain(&mut self, kp: f64, ki: f64, kd: f64)
        {
        self.kp = kp;
        self.ki = ki;
        self.kd = kd;
        }

    pub fn update(&mut self, setpoint: f64, measurement: f64) -> f64
        {
        // Compute the error signal
        let error_signal = setpoint - measurement;

        // Proportional term
        let proportional = self.kp * error_signal;

        // Integral term
        // current term depends on the previous term
        self.integrator += 0.5 * self.ki * self.t * (error_signal + self.prev_error);

        // Anti wind-up via integrator clamping 
        let mut lim_max_int: f64 = 0.0;
        if self.lim_max > proportional
            {
            lim_max_int = self.lim_max - proportional;
            }

        let mut lim_min_int: f64 = 0.0;
        if self.lim_min < proportional
            {
            lim_min_int = self.lim_min - proportional;
            }

        // clamp the integrator
        if self.integrator > lim_max_int
            {
            self.integrator = lim_max_int;
            }
        else if self.integrator < lim_min_int
            {
            self.integrator = lim_min_int;
            }

        // Derivative term (band-limited differentiator)
        self.differentiator = (2.0 * self.kd * (measurement - self.prev_measurement) +
                              (2.0 * self.tau - self.t) * self.differentiator) /
                              (2.0 * self.tau);

        // Compute the output
        self.output = proportional + self.integrator + self.differentiator;

        // Clamp the output
        if self.output > self.lim_max
            {
            self.output = self.lim_max;
            }
        else if self.output < self.lim_min
            {
            self.output = self.lim_min;
            }

        // Store error and measurement for later use
        self.prev_error = error_signal;
        self.prev_measurement = measurement;

        // Return the output
        self.output
        }
    }

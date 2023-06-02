use na::DVector;
use nalgebra::DMatrix;

pub fn kalman_filter(x: &DVector<f64>, P: &DMatrix<f64>, a: &DMatrix<f64>, B: &DMatrix<f64>, u: &DMatrix<f64>, H: &DMatrix<f64>, R: &DMatrix<f64>, z: &DVector<f64>) -> (&DVector<f64>, &DMatrix<f64>) 
    {
    // Prediction step
    let x_pred = &a * x + &b * u;
    let p_pred = &a * P * a.transpose();

    // Update step
    let y = z - &H * x_pred;

    }

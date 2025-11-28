use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LinearModel {
	pub theta0: f64,
	pub theta1: f64,
	pub mean: f64, // mean of mileage
	pub std: f64,  // std of mileage
}

impl LinearModel {
	pub fn new() -> Self {
		LinearModel {
			theta0: 0.0,
			theta1: 0.0,
			mean: 0.0,
			std: 1.0,
		}
	}

	// normalize mileage automatically for prediction
	pub fn estimate_price(&self, mileage: f64) -> f64 {
		let x_norm = (mileage - self.mean) / self.std;
		self.theta0 + self.theta1 * x_norm
	}
}

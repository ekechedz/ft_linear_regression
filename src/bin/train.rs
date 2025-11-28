use ft_linear_regression::linear_model::LinearModel;
use ft_linear_regression::utils::{read_dataset, save_model};

fn main() {
	let data = read_dataset("data.csv");

	let m = data.len() as f64;
	let learning_rate = 0.01;
	let iterations = 5000;

	let mean: f64 = data.iter().map(|(km, _)| km).sum::<f64>() / m;
	let std: f64 = (data.iter().map(|(km, _)| (km - mean).powi(2)).sum::<f64>() / m).sqrt();

	let normalized_data: Vec<(f64, f64)> = data
		.iter()
		.map(|(km, price)| ((km - mean) / std, *price))
		.collect();

	let mut model = LinearModel::new();

	for iter in 0..iterations {
		let mut sum_error_t0 = 0.0;
		let mut sum_error_t1 = 0.0;

		for (x, y) in &normalized_data {
			let prediction = model.estimate_price(*x);
			let error = prediction - y;
			sum_error_t0 += error;
			sum_error_t1 += error * x;
		}

		let tmp_t0 = learning_rate * sum_error_t0 / m;
		let tmp_t1 = learning_rate * sum_error_t1 / m;

		model.theta0 -= tmp_t0;
		model.theta1 -= tmp_t1;

		if iter % 500 == 0 {
			let loss: f64 = normalized_data
				.iter()
				.map(|(x, y)| (model.estimate_price(*x) - y).powi(2))
				.sum::<f64>()
				/ (2.0 * m);
			println!("Iteration {}: Loss = {:.4}", iter, loss);
		}
	}

	model.mean = mean;
	model.std = std;

	save_model(&model, "model.json");

	println!(
		"Training completed.\nTheta0 = {:.2}\nTheta1 = {:.6}",
		model.theta0, model.theta1
	);
}

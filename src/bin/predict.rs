use ft_linear_regression::utils::load_model;
use std::io::{self, Write};

fn main() {
	let model = load_model("model.json");

	print!("Enter mileage: ");
	io::stdout().flush().unwrap();

	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();

	let mileage: f64 = input.trim().parse().expect("Invalid number");

	let price = model.estimate_price(mileage);
	println!("Estimated price: {:.2}", price);
}

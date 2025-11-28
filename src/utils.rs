use crate::linear_model::LinearModel;
use csv::ReaderBuilder;
use serde_json;
use std::fs::File;
use std::io::Write;

pub fn read_dataset(path: &str) -> Vec<(f64, f64)> {
	let mut rdr = ReaderBuilder::new()
		.has_headers(true)
		.from_path(path)
		.expect("Cannot open dataset");

	let mut data = Vec::new();
	for result in rdr.records() {
		let record = result.expect("Invalid CSV row");
		let mileage: f64 = record[0].parse().unwrap();
		let price: f64 = record[1].parse().unwrap();
		data.push((mileage, price));
	}
	data
}

pub fn save_model(model: &LinearModel, path: &str) {
	let json = serde_json::to_string(model).unwrap();
	let mut file = File::create(path).unwrap();
	file.write_all(json.as_bytes()).unwrap();
}

pub fn load_model(path: &str) -> LinearModel {
	let file = File::open(path).expect("Model not found. Run training first.");
	serde_json::from_reader(file).unwrap()
}

	fn main() {
		let p:f64 = 510000.0;
		let r:f64 = 5.0;

		//depreciation
		let a = p * (( 1.0 - (r / 100.0)) * ( 1.0 - (r/100.0)) * ( 1.0 - (r/100.0)));
		println!("Depreciation is {}", a);

	}
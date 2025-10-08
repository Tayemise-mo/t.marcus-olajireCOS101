	fn main() {
		Let p:f64 = 520,000,000.0;
		Let r:f64 = 10.0;
		Let t:f64 = 5.0;

		//simple interest
		Let a = p * ( 1.0 + (r / 100.0)) * t;
		println!("Amount is {}", a);
		Let si = a - p;
		println!("Simple Interest is {}", si);

	}
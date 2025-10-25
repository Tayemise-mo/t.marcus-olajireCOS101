use std::io::stdin;
use std::f32;

fn main() {

    let mut exp_str = String::new();
    let mut age_str = String::new();

    println!("Enter years of experience,
        10-20 = experienced
        0-9 = inexperienced = ");
    stdin().read_line(&mut exp_str).expect("Not a valid string");
    let experience:f32=exp_str.trim().parse().expect("Not a valid numver");

    println!("Enter value of age= ");
    stdin().read_line(&mut age_str).expect("Not a valid string");
    let age:f32=age_str.trim().parse().expect("Not a valid number");

    if experience>=10.0 && age>=40.0{
        println!("Incentive of employee is N1_560_000");
    }else if experience>=10.0 && age>=30.0&& age<40.0{
        println!("Incentive of employee is N1_480_000");
    }else if experience>=10.0 && age<28.0{
        println!("Incentive of employee is N1_300_000");
    }else if experience<10.0{
        println!("Incentive of employee is 100_000");
    }
    
}

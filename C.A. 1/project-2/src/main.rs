use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value for Principal (P): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let p:f32 = input1.trim().parse().expect("Not a valid Number");

    println!("Enter value for Rate (R): ");
    io::stdin().read_line(&mut input2).expect("Not a vaild string");
    let r:f32 = input2.trim().parse().expect("Not a valid Number");

    println!("Enter Value for Time (T): ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let t:f32 = input3.trim().parse().expect("Not a valid number");

    let a = p * ((1.0 + (r/100.0))**t);
    println!("Total Amount = {}",a);

    let ci:f32 = a - p;
    println!("Compound Interest = {}", ci);

}

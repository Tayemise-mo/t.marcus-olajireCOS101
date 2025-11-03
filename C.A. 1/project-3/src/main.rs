use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Code          Item            Price(N)");
    println!("L             Laptop         550_000");
    println!("M             Monitor        120_000");
    println!("K             Keyboard       15_000");
    println!("H             Headset        25_000");

    println!("Enter item code: ",);
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let item_code:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter quantity: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quantity:f32 = input2.trim().parse().expect("Not a valid number");
    
    let L:f32 = 550_000.0;
    let M:f32 = 120_000.0;
    let K:f32 = 15_000.0;
    let H:f32 = 25_000.0;

    let item_code = L or M or K or H;

    let total_cost::f32 = item_code * quantity;

    if total_cost > 500_000.0 {
        let total_Discount_cost:f32 = (total_cost)*0.07;
        println!("Amount payable = {}", total_Discount_cost);
    }else{
    println!("Amount payable = {}", total_cost);
    }
}

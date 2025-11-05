use std::io;

fn main() {
    // 1. Display the menu
    println!("========== MENU ==========");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken      - N3,000");
    println!("A = Amala & Ewedu Soup        - N2,500");
    println!("E = Eba & Egusi Soup          - N2,000");
    println!("W = White Rice & Stew         - N2,500");
    println!("==========================");

    // 2. Get food type input
    let mut food_choice = String::new();
    println!("\nPlease enter the letter for your food choice (P, F, A, E, W):");
    io::stdin()
        .read_line(&mut food_choice)
        .expect("Failed to read line");

    // Clean and uppercase the input
    let food_choice = food_choice.trim().to_uppercase();

    // 3. Determine the price based on the choice
    // We use f64 for price to make the discount calculation easier
    let price: f32 = match food_choice.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food choice! Program will exit.");
            return; // Exit if the choice is invalid
        }
    };

    // 4. Get quantity input
    let mut quantity_str = String::new();
    println!("Enter the quantity:");
    io::stdin()
        .read_line(&mut quantity_str)
        .expect("Failed to read line");

    // Parse the quantity string to a number (f64)
    let quantity: f32 = match quantity_str.trim().parse() {
        Ok(num) => {
            if num <= 0.0 {
                println!("Quantity must be greater than zero. Program will exit.");
                return;
            }
            num
        }
        Err(_) => {
            println!("Invalid quantity! Please enter a number. Program will exit.");
            return;
        }
    };

    // 5. Calculate total and apply discount logic
    let total_order = price * quantity;
    let discount_threshold = 10000.0;
    let discount_rate = 0.05; // 5%

    let final_total: f32;

    println!("\n--- ORDER SUMMARY ---");
    println!("Initial Total: N{:.2}", total_order);

    if total_order > discount_threshold {
        let discount_amount = total_order * discount_rate;
        final_total = total_order - discount_amount;
        println!("Discount (5%): -N{:.2}", discount_amount);
        println!("---------------------");
        println!("Final Total: N{:.2}", final_total);
    } else {
        final_total = total_order;
        println!("(No discount applied. Order is not over N10,000)");
        println!("---------------------");
        println!("Final Total: N{:.2}", final_total);
    }
}
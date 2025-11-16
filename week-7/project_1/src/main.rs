use std::io;
use std::f64::consts::PI;

fn main() {
    println!("=== MTH 101 Area & Volume Calculator ===");
    println!("Select a shape to calculate:");
    println!("1. Area of a Trapezium");
    println!("2. Area of a Rhombus");
    println!("3. Area of a Parallelogram");
    println!("4. Area of a Cube");
    println!("5. Volume of a Cylinder");

    let choice = read_input("Enter your choice (1–5):")
        .trim()
        .parse::<u32>()
        .expect("Invalid input");

    match choice {
        1 => {
            let h = read_input("Enter height:").trim().parse::<f64>().unwrap();
            let b1 = read_input("Enter base1:").trim().parse::<f64>().unwrap();
            let b2 = read_input("Enter base2:").trim().parse::<f64>().unwrap();
            println!("Area of Trapezium = {}", area_trapezium(h, b1, b2));
        }

        2 => {
            let d1 = read_input("Enter diagonal1:").trim().parse::<f64>().unwrap();
            let d2 = read_input("Enter diagonal2:").trim().parse::<f64>().unwrap();
            println!("Area of Rhombus = {}", area_rhombus(d1, d2));
        }

        3 => {
            let base = read_input("Enter base:").trim().parse::<f64>().unwrap();
            let altitude = read_input("Enter altitude:").trim().parse::<f64>().unwrap();
            println!("Area of Parallelogram = {}", area_parallelogram(base, altitude));
        }

        4 => {
            let side = read_input("Enter side length:").trim().parse::<f64>().unwrap();
            println!("Area of Cube = {}", area_cube(side));
        }

        5 => {
            let r = read_input("Enter radius:").trim().parse::<f64>().unwrap();
            let h = read_input("Enter height:").trim().parse::<f64>().unwrap();
            println!("Volume of Cylinder = {}", volume_cylinder(r, h));
        }

        _ => println!("Invalid choice! Please restart the program."),
    }
}

// ---------- FUNCTIONS ----------
fn area_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    (height / 2.0) * (base1 + base2)
}

fn area_rhombus(d1: f64, d2: f64) -> f64 {
    0.5 * d1 * d2
}

fn area_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_cube(side: f64) -> f64 {
    6.0 * side.powi(2)
}

fn volume_cylinder(radius: f64, height: f64) -> f64 {
    PI * radius.powi(2) * height
}

// ---------- READ INPUT ----------
fn read_input(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input
}

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("Enter Name: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    println!("Value of First Test: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let score1:f32 = input3.trim().parse().expect("Input not an integer");

    println!("Value of Second Test: ");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let score2:f32 = input4.trim().parse().expect("Input not an integer");

    println!("Value of Third Test: ");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let score3:f32 = input5.trim().parse().expect("Input not an integer");

    let average :f32 = ((score1 + score2 + score3) / 3.0) * 100.0;

    if average >= 70.0 {
        println!("Student Name {}", input1);
        println!("Grade = A");
    }else if average >= 60.0 && average <= 69.0 {
        println!("Student Name {}", input1);
        println!("Grade = B");
    }else if average >= 50.0 && average <= 59.0 {
        println!("Student Name {}", input1);
        println!("Grade = C");
    }else if average >= 45.0 && average <= 49.0 {
        println!("Student Name {}", input1);
        println!("Grade = D");
    }else if average >= 0.0 && average <= 44.0 {
        println!("Student Name {}", input1);
        println!("Grade = F");
    }
}

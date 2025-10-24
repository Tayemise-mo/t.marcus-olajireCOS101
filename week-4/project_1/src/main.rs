use std::io::stdin;
use std::f32;

fn main() {

    let mut a_str=String::new();
    let mut b_str=String::new();
    let mut c_str=String::new();


    println!("Enter value of a: ");
    stdin().read_line(&mut a_str).expect("Not a valid string");
    let a:f32=a_str.trim().parse().expect("Not a valid number");

    println!("Enter the value of b: ");
    stdin().read_line(&mut b_str).expect("Not a valid string");
    let b:f32=b_str.trim().parse().expect("Not a valid number");

    println!("Enter value of c: ");
    stdin().read_line(&mut c_str).expect("Not a valid string");
    let c:f32=c_str.trim().parse().expect("Not a valid number");


    let d=b*b-4.0*a*c;

    if d>0.0{
        let root1=(-b+d.sqrt())/(2.0*a);
        let root2=(-b-d.sqrt())/(2.0*a);
        println!("There are two distinct roots:{}and{}", root1, root2);
    }else if d==0.0{
        let  root=-b/2.0*a;
        println!("There is exactly one real root:{}", root);
    }else{
        let real_part=-b/(2.0*a);
        let imag_part=(-d).sqrt()/(2.0*a);
        println!(
            "There are no real roots:{}+{}i and {}-{}i", real_part,imag_part,real_part,imag_part
            );
    }
}

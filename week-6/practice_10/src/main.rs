fn main() {
    let a = 20;
    let b = 30;

    if (a > 10) && (b > 10) {
        println!("true");
    }
    let c = 0;
    let d = 30;

    if (c>10) || (d>10){
        println!("true");
    }
    let is_either = false;

    if !is_either {
        println!("Not Elder");
    }
}


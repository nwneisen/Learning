fn main() {
    let mut x: u8 = 255;
    println!("x is {}", x);

    x = 20;
    println!("x is {}", x);

    let y: f32 = 2.1234567890;
    println!("y is {}", y);

    let a = 10;
    let b = 3.0;
    let c = a as f64 + b;
    println!("c is {}", c);

    let d = a as f64 * b;
    println!("d is {}", d);

    let e = a as f64 / b;
    println!("e is {}", e);
}

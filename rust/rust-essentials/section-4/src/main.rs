fn main() {
    // Compiler is smart enough to give us u8 instead of the usual i32
    // let x = 1;
    // let y = 2;
    // parameters(13, x, y);

    // return_values();
    convert_temp();
}

#[allow(dead_code)]
fn parameters(number: i32, a: u8, b: u8) {
    println!("Hello!");
    println!("number is {}", number);
    println!("Sum is {}", a+b);
}

#[allow(dead_code)]
fn return_values() {
    let result = square(13);
    println!("result is {:?}", result);
}
fn square(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x*x);
    // println!("End of function");
}

#[allow(dead_code)]
fn convert_temp() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (1.8*celsius)+32.0
}

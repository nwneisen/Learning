fn main() {
    // numbers();
    // print_formatting();
    // bitwise_ops();
    // booleans();
    // chars();
    average();
}

#[allow(dead_code)]
fn numbers() {
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
}

#[allow(dead_code)]
fn print_formatting() {
    let a = 10;
    let b = 3.0;
    let e = a as f64 / b;
    println!("e is {0:08.3}\na is {1}\nOnce again c is {0}", e, a);
}

#[allow(dead_code)]
fn bitwise_ops(){
    let mut value = 0b1111_0101u8;
    println!("value is {}", value);
    println!("value is {:08b}", value);

    value = !value;
    println!("value is {:08b}", value);

    value = value & 0b111_0111;
    println!("value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000);

    value = value | 0b0100_0000;
    println!("value is {:08b}", value);

    value = value ^ 0b0101_0101;
    println!("value is {:08b}", value);

    value = value << 4;
    println!("value is {:08b}", value);

    value = value >> 2;
    println!("value is {:08b}", value);
}

#[allow(dead_code)]
fn booleans() {
    let a = true;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a^b);

    let c = (a^b) | (a&b);
    println!("c is {}", c);

    // Short circuits and doesn't call the panic macro
    let d = (a^b) || panic!();
    println!("d is {}", d);
}

#[allow(dead_code)]
fn chars() {
    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';
    println!("{}\n{}\n{}", letter, number, finger);
}

fn average() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");
}

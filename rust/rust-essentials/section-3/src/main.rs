fn main() {
    // arrays();
    // multidimensional_arrays();
    tuples();
}

#[allow(dead_code)]
fn arrays() {
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("First_letter is {}", first_letter);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index: usize = numbers.len()-1;
    println!("last_number is {}", numbers[index]);
}

#[allow(dead_code)]
fn multidimensional_arrays() {
    let parking_lot = [
        [1,2,3],
        [4,5,6]
    ];
    let number = parking_lot[1][2];
    println!("number is {}", number);

    let garage = [[[0; 100]; 20]; 5];
    println!("{}", garage[0][0][0]);
}

#[allow(dead_code)]
fn tuples() {
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first item is {}", first_item);

    let (a,b,c) = stuff;
    println!("a is {}, b is {}, c is {}", a, b, c);
}

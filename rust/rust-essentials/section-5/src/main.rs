fn main() {
    // conditionals();
    // multiple_conditions();
    // conditional_assignment();
    // loop_loop();
    // while_loop();
    // for_loop();
    // nested_loops();
    challenge();
}

#[allow(dead_code)]
fn conditionals() {
    let x = 3;

    if x + 1 != 3 {
        println!("x + 1 is NOT 3!");
    }
}

#[allow(dead_code)]
fn multiple_conditions() {
    let x = 3;
    let y = 5;

    // Nested if
    if x > y {
        println!("x is greater than y");
    } else {
        if x < y {
            println!("x is less than y")
        } else {
            println!("x is equal to y")
        }
    }

    // else if
    if x > y {
        println!("x i greater than y");
    } else if x < y {
        println!("x is less than y")
    } else {
        println!("x is equal to y")
    }
}

#[allow(dead_code)]
fn conditional_assignment() {
    let make_x_odd = true;
    let x = if make_x_odd { 1 } else { 2 };

    // if make_x_odd {
    //     x = 1;
    // } else {
    // x = 2;
    // }

    println!("x is {}", x);
}

#[allow(dead_code)]
fn loop_loop() {
    let mut count = 0;

    let result = loop {
        if count >= 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };

    println!("Count has reached 10");
    println!("Result is {}", result);
}

#[allow(dead_code)]
fn while_loop() {
    let mut count = 0;
    let letters = ['a', 'b', 'c'];

    while count < 10 {
        count += 1;
        println!("count is {}", count);
    }

    count = 0;
    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;
    }
}

#[allow(dead_code)]
fn for_loop() {
    let message = ['h', 'e', 'l', 'l', 'o'];

    for (index, &item) in message.iter().enumerate() {
        println!("{}: item is {}", index, item);
        if item == 'e' {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {}", number);
    }
}

#[allow(dead_code)]
fn nested_loops() {
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num);
        }
        println!();
    }
}

#[allow(dead_code)]
fn challenge() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    let mut mean: f64 = 0.0;

    for num in numbers {
        if num > max {
            max = num;
        }
        if num < min {
            min = num
        }
        mean += num as f64;
    }
    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!")
}

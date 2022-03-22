use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading from file");
    let lines: Vec<_> = input.lines().collect();

    let mut length: i32 = 0;
    let mut ones: [i32; 12] = [0; 12];

    for (i, line) in lines.iter().enumerate() {
        length = i as i32;

        for (j, character) in line.chars().enumerate() {
            if character == '1' {
                ones[j] += 1;
            }
        }
    }

    let mut gamma_rate: u16 = 0;
    let mut epsilon_rate: u16 = 0;
    for one in ones {
        gamma_rate <<= 1;
        epsilon_rate <<= 1;
        if length + 1 - one <= 500 {
            gamma_rate |= 1;
            epsilon_rate |= 0;
        } else {
            gamma_rate |= 0;
            epsilon_rate |= 1;
        }
    }

    println!("{:#016b}", gamma_rate);
    println!("{:#016b}", epsilon_rate);
    let power_rate: u32 = gamma_rate as u32 * epsilon_rate as u32;
    println!("{}", power_rate);
}

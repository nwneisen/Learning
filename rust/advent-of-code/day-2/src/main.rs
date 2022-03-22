use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading input from file");
    let lines: Vec<_> = input.lines().collect();

    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        let direction = line.split(" ").nth(0).unwrap();
        let distance = line.split(" ").nth(1).unwrap();

        let magnitude = distance.parse::<i32>().unwrap();
        if direction == "forward" {
            position += magnitude;
            depth += magnitude * aim;
        } else if direction == "up" {
            aim -= magnitude;
        } else if direction == "down" {
            aim += magnitude;
        }
    }

    println!("{}", position * depth);
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let max_number = &args[1].parse::<i32>().unwrap();
    let mut result = 1;

    for i in 2..=*max_number {
        result = result * i;
    }

    println!("The factorial is {}", result);
}

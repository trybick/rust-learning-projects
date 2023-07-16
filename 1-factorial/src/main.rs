use std::io;

fn factorial(number: u64) -> u64 {
    let mut result = 1;
    for i in 2..=number {
        result = result * i;
    }
    result
}

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");

    let number = input.trim().parse::<u64>().expect("That's not a number");

    let result = factorial(number);

    println!("The factorial {} is {}", input, result);


    // let args: Vec<String> = env::args().collect();
    // let max_number = &args[1].parse::<i32>().unwrap();
    // let mut result = 1;

    // for i in 2..=*max_number {
    //     result = result * i;
    // }

    // println!("The factorial is {}", result);
}

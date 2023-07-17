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
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => println!("error: {error}")
    }

    let number = input.trim().parse::<u64>().expect("That's not a number");
    let result = factorial(number);
    println!("The factorial of {} is {}", input.replace('\n', ""), result);
}

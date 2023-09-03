use rand::Rng;

// Guessing game. ask the user to guess a number between 1 and a 100.
// If you guessed correctly, it will say you win.
// If you're too high or too low it will also let you know.

fn main() {
    let answer = rand::thread_rng().gen_range(1..100);
    println!("{}", answer);

    println!("Try to guess the correct number! Please enter a number from 1 to 100: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input_number: i32 = input.trim().parse().unwrap();

    println!();
    if input_number == answer {
        println!("You win!");
    } else if input_number > answer {
        println!("Too high!");
    } else {
        println!("Too low!");
    }
}

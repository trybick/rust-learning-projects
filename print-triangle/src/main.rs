use dialoguer::{theme::ColorfulTheme, Select};

// 1. Write a programme that prints out a triangle from largest to smallest; user inputs the largest number
// 2. Write a programme that prints out a triangle from smallest to largest; user inputs bottom number.
// 3. Print out a triangle from smallest to largest, skipping even rows. User inputs largest number

fn largest_to_smallest(triangle_size: i32) {
    for n in (1..=triangle_size).rev() {
        println!("{}", "*".repeat(n.try_into().unwrap()));
    }
}

fn smallest_to_largest(triangle_size: i32) {
    for n in 1..=triangle_size {
        println!("{}", "*".repeat(n.try_into().unwrap()));
    }
}

fn smallest_to_largest_skip_even(triangle_size: i32) {
    for n in 1..=triangle_size {
        if n % 2 != 0 {
            println!("{}", "*".repeat(n.try_into().unwrap()));
        }
    }
}


fn main() {
    let options = &[
        "Print a triangle from largest to smallest",
        "Print a triangle from smallest to largest",
        "Print a triangle from smallest to largest, skipping even rows"
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option to:")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();
    let is_largest_to_smallest = selection as i32 == 0;
    let is_smallest_to_largest = selection as i32 == 1;

    let mut triangle_size = String::new();
    println!("Enter a number for size of triangle:");
    std::io::stdin().read_line(&mut triangle_size).unwrap();
    let triangle_size_number: i32 = triangle_size.trim().parse().unwrap();

    println!();
    if is_largest_to_smallest {
        largest_to_smallest(triangle_size_number)
    } else if is_smallest_to_largest {
        smallest_to_largest(triangle_size_number)
    } else {
        smallest_to_largest_skip_even(triangle_size_number)
    }
}

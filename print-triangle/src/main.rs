use dialoguer::{theme::ColorfulTheme, Select};

// 1. Write a programme that prints out a triangle from largest to smallest; user inputs the largest number
// 2. Write a programme that prints out a triangle from smallest to largest; user inputs bottom number.
// 3. Print out a triangle from smallest to largest, skipping even rows. User inputs largest number


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
    let largest_to_smallest = selection as i32 == 0;
    let smallest_to_largest = selection as i32 == 1;

    let mut triangle_size = String::new();
    println!("Enter a number for size of triangle:");
    std::io::stdin().read_line(&mut triangle_size).unwrap();
    let triangle_size_number: f64 = triangle_size.trim().parse().unwrap();

    if largest_to_smallest {
        println!();
        println!("largest to smallest");
    } else if (smallest_to_largest) {
        println!();
        println!("smallest to largest");
    } else {
        println!();
        println!("smallest to largest, skipping even rows");
    }
}

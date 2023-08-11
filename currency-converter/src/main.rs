use dialoguer::{theme::ColorfulTheme, Select};

const EXCHANGE_RATE_USD_TO_CHINESE: f64 = 7.24;
const EXCHANGE_RATE_CHINESE_TO_USD: f64 = 0.14;


fn main() {
    let options = &[
        "USD to Chinese Yuan",
        "Chinese Yuan to USD",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option to convert:")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();
    let is_usd_to_chinese = selection as i32 == 0;

    let mut amount = String::new();
    println!("Enter amount:");
    std::io::stdin().read_line(&mut amount).unwrap();
    let amount_number: f64 = amount.trim().parse().unwrap();

    if is_usd_to_chinese {
        let result = amount_number * EXCHANGE_RATE_USD_TO_CHINESE;
        println!();
        println!("{} USD converted to Chinese Yuan is {:.2}", amount.trim(), result);
    } else {
        println!();
        let result = amount_number * EXCHANGE_RATE_CHINESE_TO_USD;
        println!("{} Chinese Yuan converted to USD is {:.2}", amount.trim(), result);
    }
}

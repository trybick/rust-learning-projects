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

    if is_usd_to_chinese {
        let result = amount.parse::<f64>().unwrap() * EXCHANGE_RATE_USD_TO_CHINESE;
        println!("{} USD converted to Chinese Yuan is {}", amount, result);
    } else {
        let result = amount.parse::<f64>().unwrap() * EXCHANGE_RATE_CHINESE_TO_USD;
        println!("{} Chinese Yuan converted to USD is {}", amount, result);
    }

}

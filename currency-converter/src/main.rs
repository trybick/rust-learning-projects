use dialoguer::{theme::ColorfulTheme, Select};

// Develop a programme to convert currency X to currency Y and vice versa
//
// Input - ask do you want to convert FROM or TO currency
// Input - enter amount
// Do the conversion and output it


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

}

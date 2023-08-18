mod calc;
mod weather;
mod coin;
mod gen_passwd;
mod tfi;
mod cc;
use clap::Parser;

/// Hello, World!
#[derive(Parser, Debug)]
enum Hw {
    /// Use the calculator  
    #[clap(name = "calculator", alias = "c")]
    Calculator,

    /// Check the weather
    #[clap(name = "weather", alias = "w")]
    Weather {
        /// Specify the city
        #[arg()]
        city: Option<String>,
    },

    /// Flip a coin
    #[clap(name = "flip", alias = "f")]
    Flip,

    /// Generate a password
    #[clap(name = "genpasswd", alias = "g")]
    GenPasswd {
        /// Stronger pass (using symbols)
        #[clap(long, short)]
        symbol: bool,
    },

    /// Prints file size, lines, words, word occurrences of an text file
    #[clap(name = "TextFileInfo", alias = "t")]
    TxtFileInfo {
        /// Specify the text file
        #[arg()]
        file_path: String,

        /// Prints the contents of the file
        #[clap(long, short)]
        print_file: bool,

        /// Prints the number of occurrences of each word in the file
        #[clap(long, short)]
        word_occur: bool,
    },

    /// Convert one currency to another
    #[clap(name = "currencyConvert", alias = "cc")]
    Cc {
        /// list all avalable currencies
        #[clap(long, short)]
        list: bool,

        /// amount
        #[clap()]
        amount: Option<f64>,

        /// currency from
        #[clap()]
        base: Option<String>,

        /// currency to
        #[clap()]
        to: Option<String>,
    }
}


fn main() {
    let args = Hw::parse();

    match args {
        Hw::Calculator => calc::main(),
        Hw::Weather { city: None } => weather::get_weather(),
        Hw::Weather { city: Some(city) } => weather::get_weather_city(&city),
        Hw::Flip => coin::main(),
        Hw::GenPasswd { symbol } => gen_passwd::main(symbol),
        Hw::TxtFileInfo {file_path, print_file, word_occur } => tfi::main(file_path, print_file, word_occur),
        Hw::Cc { list: false, amount: Some(amount), base: Some(base), to: Some(to)} => cc::main(amount, base.as_str(), to.as_str()),
        Hw::Cc { list: true, amount: _, to: _, base: _ } => cc::list_currencies(),
        _ => println!("false"),
    }
}
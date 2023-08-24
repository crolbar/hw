mod calc;
mod weather;
mod coin;
mod gen_passwd;
mod tfi;
mod cc;
mod notes;
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

    /// Convert one currency to another (e.g: `hw cc 15 USD EUR`)
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
    },

    /// Notes
    #[clap(name = "Notes", alias = "n")]
    Notes {
        /// name of the note
        #[arg(default_value = "-")]
        name: Option<String>,

        /// list all the notes
        #[arg(short, long)]
        list_notes: bool,

        /// print the contents of a note
        #[arg(short, long)]
        cat_note: bool,
        
        /// remove a note
        #[arg(short, long)]
        del_note: bool,
    },
}


fn main() {
    let args = Hw::parse();

    match args {
        Hw::Calculator => calc::main(),
        Hw::Flip => coin::main(),

        Hw::GenPasswd { symbol } => gen_passwd::main(symbol),

        Hw::Weather { city: None } => weather::get_weather(),
        Hw::Weather { city: Some(city) } => weather::get_weather_city(&city),

        Hw::TxtFileInfo {file_path, print_file, word_occur } => tfi::main(file_path, print_file, word_occur),

        Hw::Cc { list: false, amount: Some(amount), base: Some(base), to: Some(to)} => cc::main(amount, base.as_str(), to.as_str()),
        Hw::Cc { list: true, amount: _, to: _, base: _ } => cc::list_currencies(),

        Hw::Notes { name: Some(name), list_notes: false, cat_note: false, del_note: false} => notes::create_note(name),
        Hw::Notes { name: Some(name), list_notes, cat_note, del_note } => notes::lcr_note(name, list_notes, cat_note, del_note),

        _ => println!("Invalid command or argument use -h or --help for help."),
    }
}
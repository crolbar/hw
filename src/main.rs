mod calc;
mod weather;
mod coin;
mod gen_passwd;
use clap::Parser;


/// Hello, World.
#[derive(Parser, Debug)]
enum Hw {
    /// Use the calculator
    #[clap(name = "calculator", alias = "c")]
    Calculator,

    /// Use the weather app
    #[clap(name = "weather", alias = "w")]
    Weather {
        /// Specify the city
        #[arg(alias = "c")]
        city: Option<String>,
    },

    /// Flip a coin
    #[clap(name = "flip", alias = "f")]
    Flip,

    /// Generate a random password
    #[clap(name = "genpasswd", alias = "gp")]
    GenPasswd {
        /// Stronger pass (using symbols)
        #[clap(long, short)]
        symbol: bool,
    },
    
}


fn main() {
    let args = Hw::parse();

    match args {
        Hw::Calculator => calc::main(),
        Hw::Weather { city: None } => weather::get_weather(),
        Hw::Weather { city: Some(city) } => weather::get_weather_city(&city),
        Hw::Flip => coin::main(),
        Hw::GenPasswd { symbol } => gen_passwd::main(symbol),
    }
}
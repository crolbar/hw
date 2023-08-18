use reqwest;
extern crate tokio;
extern crate serde_json;
use serde_json::Value;

async fn fetch_exchange_rate(base: &str, to: &str) -> Result<String, reqwest::Error> {
    let api_key = "fca_live_dKwmB9vs3KidPYY4lsZ1QpAh6br5m5q3ow5EGrqN";

    let url = format!("https://api.freecurrencyapi.com/v1/latest?apikey={}&currencies={}&base_currency={}", api_key, to, base);

    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

#[tokio::main]
async fn print_cc(base: &str, to: &str, amount: f64) {
    let response = fetch_exchange_rate(base, to).await;

    match response {
        Ok(data) => {
            // println!("{}", data);
            let json_result: Value = serde_json::from_str(&data).unwrap_or_else(|_| {
                println!("Error parsing JSON response.");
                serde_json::Value::Null
            });

            if let Some(exrate) = json_result["data"][to].as_f64() {
                println!("exchange rate: {}", exrate);
                let converted_currency = amount * exrate;
                println!("{} {} to {} = {:.2} {}", amount, base, to, converted_currency, to)
            } else { println!("Some required fields are missing in the JSON response.") }
        }
        Err(e) => println!("{}", e),
    }
}

pub fn main(amount: f64, base: &str, to: &str) {
    print_cc(base, to, amount);
}

pub fn list_currencies() {
println!("
EUR	Euro
USD	US Dollar
JPY	Japanese Yen
BGN	Bulgarian Lev
CZK	Czech Republic Koruna
DKK	Danish Krone
GBP	British Pound Sterling
HUF	Hungarian Forint
PLN	Polish Zloty
RON	Romanian Leu
SEK	Swedish Krona
CHF	Swiss Franc
ISK	Icelandic Króna
NOK	Norwegian Krone
HRK	Croatian Kuna
RUB	Russian Ruble
TRY	Turkish Lira
AUD	Australian Dollar
BRL	Brazilian Real
CAD	Canadian Dollar
CNY	Chinese Yuan
HKD	Hong Kong Dollar
IDR	Indonesian Rupiah
ILS	Israeli New Sheqel
INR	Indian Rupee
KRW	South Korean Won
MXN	Mexican Peso
MYR	Malaysian Ringgit
NZD	New Zealand Dollar
PHP	Philippine Peso
SGD	Singapore Dollar
THB	Thai Baht
ZAR	South African Rand");
}
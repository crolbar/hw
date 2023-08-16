use rand::Rng;
use std::io;

fn generate_password(length: usize, symbols: bool) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+[]{}|;:,.<>?";
    let charset_length;
    if symbols {charset_length = CHARSET.len();} else {charset_length = CHARSET.len() - 26;}

    let password: String = (0..length).map(|_| {
            let idx = rand::thread_rng().gen_range(0..charset_length);
            CHARSET[idx] as char
        })
        .collect();

    password
}

pub fn main(symbols: bool) {
    println!("Enter the length of the password:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let length: usize = match input.trim().parse() { Ok(p) => p, Err(_) => { println!("Invalid input."); return; } };

    let password = generate_password(length, symbols);

    println!("{}", password);
}
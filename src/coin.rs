pub fn main() {
    let bool = rand::random::<bool>();
    match bool {
        true => println!("heads"),
        false => println!("tails"),
    }
}

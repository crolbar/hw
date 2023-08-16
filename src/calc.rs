use std::io;

pub fn main() {
        println!("Enter something to calculate (e.g., 2 + 5):");

    loop {

        // make a new string variable and put the user input into that variable
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("crashed");

        // make an iterator and format the user input into it
        let mut iter = input.trim().split_whitespace();
        
        // convert the first value of the iterator into a float 
        let num1: f64 = match iter.next() {
            Some(num) => num.parse().expect("Invalid number"),
            None => {
                println!("Invalid input");
                continue;
            }
        };

        // check if the second value of the iterator is an action that can be done to two numbers and assign the right action
        let action = match iter.next() {
            Some(op) if op == "+" || op == "a" => "+",
            Some(op) if op == "-" || op == "s" || op == "mi"  => "-",
            Some(op) if op == "*" || op == "m" => "*",
            Some(op) if op == "/" ||op == "d" => "/",
            Some(op) if op == "^" || op == "e" => "^",
            _ => {
                println!("Invalid action. The action may be: +, a, -, s, mi, *, m, /, d, ^, e");
                continue;
            }
        };

        // convert the third value of the iterator into a float
        let num2: f64 = match iter.next() {
            Some(num) => num.parse().expect("Invalid number"),
            None => {
                println!("Invalid input");
                continue;
            }
        };
        
        // make the calculation based on the right action
        let result = match action {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => { 
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Division by zero in not a thing.");
                    continue;
                }
            }
            "^" => num1.powf(num2),
            _ => continue,
        };

        // print the result of the calculation
        println!("resut: {result}");
    }
}

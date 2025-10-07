use std::io;

fn main() {
    fahrenheit_to_celsius();
    nth_fibonacci_number();
}

fn fahrenheit_to_celsius() {
    loop {
        let mut input = String::new();
        println!("Enter Fahrenheit temperature:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let fahrenheit: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("Celsius temperature: {}", celsius);
        break;
    }
}

fn  nth_fibonacci_number() {
    loop {
        let mut input = String::new();
        println!("Enter the position of the Fibonacci sequence:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let n: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        let fibonacci = fibonacci(n);
        println!("Fibonacci number at position {}: {}", n, fibonacci);
        break;
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

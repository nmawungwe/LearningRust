use std::io;
// use rand::Rng;

fn main() {
    println!("Welcome to the nth Fibonacci calculator");

    loop {

        println!("Please input the nth number.");

        let mut number = String::new();
    
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line!");
    
        let number: f64 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        let fibonacci_num: f64; 

        fibonacci_num = ((1.618034_f64).powf(number)-(1_f64-1.618034_f64).powf(number))/(5_f64.sqrt());

        // for n in number - 2 .. number {
        //     fibonacci_num = fibonacci_num + n
        // }


        println!("Fibonacci number of {} is {}", number, fibonacci_num);

    }

}

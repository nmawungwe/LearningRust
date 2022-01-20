use std::io;

fn main() {

    println!("Degrees to Fahrenheit converter");
    
    loop {

        println!("Input temperature in degrees celcius ");

        let mut temp = String::new();
    
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line!");
        
        let temp: f64 = match temp.trim().parse() {
            Ok(temp) => temp,
            Err(_) => continue,
        };

        let temp_fhr = (temp * 1.8) + 32.0; 

        println!("{} degrees celcius = {} degrees fahrenheit", temp, temp_fhr);

    };

}

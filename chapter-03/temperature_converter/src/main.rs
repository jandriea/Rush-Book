use std::io;

fn main() {
    println!("Fahrenheit and Celcius Converter!");
    loop {
        println!("Select mode:");
        println!("1> Fahrenheit to Celcius");
        println!("2> Celcius to Fahrenheit");
        println!("3> Quit");

        let mut mode = String::new();
        io::stdin().read_line(&mut mode)
            .expect("Failed to read line");
        let mode: u32 = match mode.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
        
        if mode == 1 {
            println!("Enter the temperature in Fahrenheit");
            let mut f_value = String::new();
            io::stdin().read_line(&mut f_value)
                .expect("Failed to read line");
            let f_value: i32 = match f_value.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a number!");
                    continue;
                }
            };
            let temp_c : f32 = (f_value as f32 - (32.0)) * (5.0/9.0);
            println!("Temperature value in Celcius is {}{}C", temp_c, '\u{00B0}');
        } else if mode == 2 {
            println!("Enter the temperature in Celcius");
            let mut f_value = String::new();
            io::stdin().read_line(&mut f_value)
                .expect("Failed to read line");
            let f_value: i32 = match f_value.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a number!");
                    continue;
                }
            };
            let temp_c : f32 = (f_value as f32 * (9.0/5.0)) +32.0;
            println!("Temperature value in Fahrenheit is {}{}F", temp_c, '\u{00B0}');
        }
        else {
            break;
        }
    }
}
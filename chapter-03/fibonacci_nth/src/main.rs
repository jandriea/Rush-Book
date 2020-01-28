fn main() {
    println!("Fibonacci n-th number!!");
    println!("Enter number");
    let mut value = String::new();
    std::io::stdin().read_line(&mut value)
        .expect("Failed to read line");
    let value : u32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            0
        }
    };
    println!("Fibonacci number {} is {}", value, fib(value));
}   

fn fib(_val : u32) -> u32 {
    if _val <= 1 {
        return _val;
    } else {
        return fib(_val-1) + fib(_val-2);
    }
    
}
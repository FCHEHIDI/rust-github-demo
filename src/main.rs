fn main() {
    println!("Hello from Rust on GitHub Codespaces!");
    println!("🦀 Rust version: {}", env!("CARGO_PKG_VERSION"));
    
    // Example: Calculate and display Fibonacci numbers
    println!("\nFirst 10 Fibonacci numbers:");
    for i in 0..10 {
        println!("F({}) = {}", i, fibonacci(i));
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
fn main() {
    println!("Hello, Rust!");
    
    // Example: Simple calculation
    let x = 5;
    let y = 10;
    let sum = x + y;
    
    println!("The sum of {} and {} is {}", x, y, sum);
    
    // Example: Vector operations
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    
    println!("Original: {:?}", numbers);
    println!("Doubled: {:?}", doubled);
}

fn main() {
    // Create variable x
    let x = 5;
    println!("The value of x is: {}", x);
    
    // Shadow the initial value of x
    let x = x + 14;
    
    // In a different scope, shadow the value again for that scope
    {
        let x = 2;
        println!("The value of x is: {}", x);
    }
    
    println!("The value of x is: {}", x);
}

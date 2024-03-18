fn main() {
    let x = 1337;           //immutable per default - Read Only afterward
    let mut y = 3321;       //makes the variable mutable - Read & Write

    println!("The current value of x is {x}");
    println!("The current value of y is {y}");

    y = -56;
    println!("The current value of y is now {y}");
}
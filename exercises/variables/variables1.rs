// variables1.rs
// Make me compile!
// Execute `rustlings hint variables1` or use the `hint` watch subcommand for a hint.


fn main() {
    let x = 5; // immutable
    println!("x has the value {}", x);
    //x = 2; // this is not allowed

    let mut y = 10; // mutable
    println!("y is mutable {}", y);
    y = 15; // allowed
    println!("y after mutating value {}", x);

    let x = 2; // shadowing allowed
    println!("x is immut but can be shadowed: {}", x);

    let x = x * x; // also allowed
    println!("x is immut but can be shadowed: {}", x)
}

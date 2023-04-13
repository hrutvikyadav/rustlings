// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

// function body needs a tail(expression without `;`) or explicit return, otherwise it will implicitly return unit type `()`
fn square(num: i32) -> i32 {
    // remove ; or use return in front of last statement to return it
    //return num * num;
    num * num
}

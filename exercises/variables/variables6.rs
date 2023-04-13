// variables6.rs
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a hint.


const NUMBER: i32 = 3; // type annotation and value needs to be present for CONSTANTS
fn main() {
    println!("Number {}", NUMBER);
    let unusable_num = 1;

    fn p_fn(num: i32, num2: i32) {
        println!("Constant Accessible here?: {}", NUMBER);
        println!("let variable not Accessible here. Need to use `closure` to access these: "/*{}", unusable_num*/);
        println!("let variable Accessible here? if passed as param: {}", num2)
    }
    p_fn(NUMBER, unusable_num); // vars got Copied as integers implement Copy trait, still usable

    println!("variables were copied into function, still free for use: {}, {}", NUMBER, unusable_num)
}

// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.


fn main() {
    let my_name = "Hrutvik";
    let my_l_name = String::from("Yadav");
    println!("Hello {}!, I am {} {}", "Rustaceans", my_name, my_l_name);

    // both variables still available here as println!() borrows value without taking ownership
    println!("1. {}>>{}", my_name, my_l_name);
    some_fn(my_l_name); // my_l_name is moved into some_fn as it takes ownership

    // my_l_name is no longer valid
    println!("2. {}>>{}", my_name, my_name/*_l_name*/);

    fn some_fn(lname: String) {
        println!("inside fn. {}>>{}", lname, lname);
    }// lname goes out of scope and drop is called to free the associated memory
}

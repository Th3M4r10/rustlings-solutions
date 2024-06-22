// intro2.rs
//
// Make the code print a greeting to the world.
//
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let a = String::from("world");  // @dev added `String::from("world");`
    println!("Hello {}!", &a);
}

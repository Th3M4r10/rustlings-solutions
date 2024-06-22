// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


#[macro_use]           // we can use this also to make it available
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    fn call_my_macro() {
        my_macro!();           // @m4r10 created function inside the mod instead of taking macro from out of the module.
    }
}

fn main() {
    // my_macro!();
}

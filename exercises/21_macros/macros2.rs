// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {       // @m4r10 changed the position of macro usage. Macro must define before we use them.
    () => {
        println!("Check out my macro!");  
    };
}

fn main() {
    my_macro!();
}



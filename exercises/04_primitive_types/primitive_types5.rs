// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let cat = ("Furry McFurson", 3.5);
    let a = cat;                    // @m4r10 corrected by creating tuple `a` and using `a.0` for name and `a.1` for age.

    println!("{} is {} years old.", a.0, a.1);  
}

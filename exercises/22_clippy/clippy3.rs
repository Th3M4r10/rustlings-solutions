// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // if my_option.is_none() {         // @m4r10 removed because of no use to return none here.
     //   my_option.unwrap();
    // }

    let my_arr = &[
        -1, -2, -3,                     // added comma(,)
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}",());

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    std::mem::swap(&mut value_a, &mut value_b);             // @m4r10 added swapping here
    println!("value a: {}; value b: {}", value_a, value_b);
}

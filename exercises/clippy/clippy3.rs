// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.

// I AM DONE

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None; //Option is unit type or none
    if my_option.is_some() { //Program panics at unwraping None. Sugestion handling it OR custom panic, as suggested 
        //my_option.unwrap(); clippy wont allow the unwrap into a unit type Some (noted as "()"), you can hide it or add a value on the variable above. I choose to simply remove.
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}

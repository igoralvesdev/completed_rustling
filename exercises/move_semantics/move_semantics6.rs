// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

// I AM DONE

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    let data = data.chars().last().unwrap();
    println!("{}", data);
    data
}

// Should take ownership 
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
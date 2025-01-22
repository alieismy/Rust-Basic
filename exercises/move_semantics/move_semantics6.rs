fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data); // Pass a reference to get_char
    println!("Last character: {}", last_char);

    string_uppercase(data); // Pass ownership to string_uppercase
}

// Should not take ownership, so take a reference
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase().to_string(); // Convert to uppercase and take ownership
    println!("{}", data);
}
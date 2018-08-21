use std::io;

fn pig_latonize(string: &mut String) -> String {
    let mut string: String = string.trim().to_string();
    let first_char: char = string.chars().next().unwrap();

    string.replace_range(0..1, "");
    string.push_str(&format!("{}ay", first_char));
    string.to_string()
}

fn main() {
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read input.");

    let user_message: Vec<&str> = user_input.trim().split(" ").collect();
    let mut final_message: Vec<String> = Vec::new();

    for element in &user_message {
        final_message.push(pig_latonize(&mut element.to_string()));
    }

    for element in &final_message {
        print!("{} ", element);
    }
}
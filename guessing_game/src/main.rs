use std::io;

fn main() {
    println!("The guessing game!");
    println!("Please enter a number:");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .ok()
        .expcet("Failed to read file!");
    println!("You enter number {}",guess);
}

// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "3"; // don't change this line
    println!("Number {}", number);

    // Shadowing (String to i32) -- use 'let' keyword
    let number = 3;

    println!("Number {}", number);
}

// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.

use std::io;

fn main() {
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("failed to read name");
    println!("Hello {username}!");
}

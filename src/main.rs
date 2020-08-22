use std::io;

fn main() {
    println!("Learning Programming Languages");
    println!("rust-basics");

    println!("Reading from stdin");
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");


    println!("User entered {}" , buffer);


}
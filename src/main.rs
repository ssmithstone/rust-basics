use std::io;

fn main() {
    println!("Learning Programming Languages");
    println!("rust-basics");

    println!("Reading from stdin");
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let res = buffer.trim().parse::<u32>();
    let res = match res {
        Ok(res) => res,
        Err(error) => panic!("Error parsing {:?}" , error),
    };

    let res2 : u32 = buffer.trim().parse().unwrap();
    let res3 : u32 = buffer.trim().parse().expect("Error parsing ");

    println!("User entered {} {} {}" , res , res2, res3);

}
fn main() {
    println!("Learning Programming Languages");
    println!("rust-basics");

    let a: u8 = 123; // bind a to value 123 8bits , 1byte. u = unsigned (positive number) , signed ( negative / positive )
    let b: i8 = -127; // signed 8bit value -127 - 128

    println!("a = {} , b = {}", a, b); // print macro with value interpolation.

    let mut c: i16 = 0; //mutable value
    println!("c = {}", c);
    c = 42;
    println!("c = {}", c);
}

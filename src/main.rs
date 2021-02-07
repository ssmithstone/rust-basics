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

    let d = 123456789; // non typed variable
    println!("d = {}" , d);
    println!("size = {} bytes , 1 byte = 8 bits" , std::mem::size_of_val(&d));

    let z:isize = 123; // size of the word/pointer
    let size_of_z = std::mem::size_of_val(&z);
    println!("z = {} , takes up {} bytes , {}-bit os" , z , size_of_z , size_of_z * 8);
}

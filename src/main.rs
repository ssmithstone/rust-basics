#![allow(dead_code)]

mod sh;

static Z:i32 = 123;

fn main() {
    println!("Learning Programming Languages");
    println!("rust-basics");

    sh::stack_and_heap();
}

fn _scope_and_shadowing() {
    let a = 123;

    {
        let a = 777;
        println!("inside , a = {}" , a);
    }

    println!("outside , a = {}", a);
}

fn _fundamental_data_types() {
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

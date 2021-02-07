#![allow(dead_code)]

mod sh;

static Z:i32 = 123;

fn if_statements(){

    let temp = 35;
    if temp > 30 {
        println!("Hot outside");
    }
    else if temp < 10 {
        println!("Wrap up it's cold")
    }
    else {
        println!("just right");
    }

    let day = if temp > 20 { "sunny"} else { "cloudy"};
    println!("day = {}" , day);

    println!("it is {}" ,
        if temp > 20 { "hot"} else if temp < 10 { "cold" } else { "ok"}
    );

    println!("it is {}" ,
        if temp > 20 {
            if temp > 30 { "very hot"} else { "hot "}
        } else if temp < 10 { "cold" } else { "ok"}
    )
}

fn while_loop(){
    let mut a = 10;
    while a > 0 {
        println!("Inside loop at {}" , a);
        a -= 1;
    }
}

fn for_loop() {
    for x in 1..11 {
        println!("x = {}" , x);
    }
}

fn main() {
    println!("Learning Programming Languages");
    println!("rust-basics");

    for_loop();
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

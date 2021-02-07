#![allow(dead_code)]
struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{ x : 0.0 , y : 0.0}
}

pub fn stack_and_heap(){
    let p1 = origin(); // stack allocation

    let p2 = Box::new(origin()); // heap allocation

    println!("stack size p1 = {} " , std::mem::size_of_val(&p1));
    println!("stack size p2 = {} " , std::mem::size_of_val(&p2));
    println!("stack size *p2 = {} " , std::mem::size_of_val(&*p2));

    println!("Point x = {} , y = {}" , (*p2).x , (*p2).y);
}
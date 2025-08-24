use std::{collections::HashMap, fs};

//This file might heavily change
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

struct Point<T> {
    x: T,
    y: T,
}

// enum Try <A, B> {
//     Ok(A),
//     Err(B),
// }

fn main() {
    println!(
        "The area of the circle is: {}",
        calc_area(Shape::Circle(5.0))
    );

    println!(
        "The area of the square is: {}",
        calc_area(Shape::Square(5.0))
    );

    println!(
        "The area of the rectangle is: {}",
        calc_area(Shape::Rectangle(5.0, 10.0))
    );

    let point: Point<i32> = Point { x: 5, y: 10 };

    println!("The coordinates of the point are: ({}, {})", point.x, point.y);

    let file = fs::read_to_string("./public/text.txt");

    match file {
        Ok(content) => {
            println!("File content: {}", content);
        }
        Err(error) => {
            println!("Error reading file: {}", error);
        }
    }

let random_u8: u8 = rand::random();


println!("char: '{}'", random_u8 as i32);

let stt = String::from("Vaibhav");

doSomething(&stt);

let stt2 = &stt;

println!("String after function call: {}", stt2);

// value ownership is now doSomething
println!("String after function call: {}", stt);

let mut vec = Vec::new();

vec.push(1);
vec.push(2);
vec.push(3);

println!("Vector: {:?}", vec);


//HashMaps

let mut users = HashMap::new();

users.insert("Vaibhav", 25);
users.insert("Alice", 30);
users.insert("Bob", 22);

match users.get("Vaibhav") {
    Some(age) => println!("Vaibhav's age is: {}", age),
    None => println!("Vaibhav not found"),

}
}

fn doSomething (s : &String){
    println!("{}", s);
}

fn calc_area(shape: Shape) -> f64 {
    let ans: f64 = match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,

        Shape::Rectangle(width, height) => width * height,

        Shape::Square(side) => side * side,
    };

    return ans;
}

# Here y is a int datatype (as x+1 do not have ;)

    fn main() {
        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {y}");
    }

# Here y is a function (as x+1 has ;)

    fn main() {
        let y = {
            let x = 3;
            x + 1;
        };

        println!("The value of y is: {y}");
        //Error
    }


##### Clone: .clone()

# enms

    enum Shape {
        Circle(f64),
        Square(f64),
        Rectangle(f64, f64),
    }

     let ans: f64 = match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,

        Shape::Rectangle(width, height) => width * height,

        Shape::Square(side) => side * side,
    };

# stuct

    struct Point<T> {
        x: T,
        y: T,
    }

 T can be of any type.

 # try-catch

    match file{
        Ok (some) =>{
            func
        }
        Err (some) =>{
            func
        }
    }

# Optional

    match str {
        Some (s) => {
            func
        }
        None => {
            func
        }
    }


# Array

    let mut vec = Vec::new();
    vec.push(random);

# Hashmaps

    let mut hash = HashMap::new();
    hash.insert(k: String::from("Vaibhav"), v: 20);
    hash.get("vaibhav");

# Print array or hashmap

    print!("{:?}", vec );
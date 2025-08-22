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
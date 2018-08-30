fn main() {
    println!("Hello, world!");

    let tuple: (i32, bool, f64) = (7, false, 3.51);

    function();
    function_with_arguments(10, 5.7533);

    let result = function_with_return(tuple);
    println!("{}", result);
}

// function
fn function() {
    println!("another function");
}

// function with arguments
fn function_with_arguments(x: u32, f: f64) {
    println!("arguments: {}, {}", x, f);
}

// function with tuple argument
fn function_with_return(tuple: (i32, bool, f64)) -> String {
    // destructure tuple
    let (a,b,c) = tuple;
    // println!("{} {} {}", a, b, c);
    return format!("{} {} {}", a, b, c);
}
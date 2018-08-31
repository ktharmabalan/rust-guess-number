fn main() {
    
    let i : i32 = 5;
    println!("i: {}", i);
    take_owner_ship(i);
    // integer is copied
    println!("i after: {}", i);
    
    let s = String::from("Hello");
    println!("s: {}", s);

    // String is moved
    take_owner_ship_string(s);
    // println!("{}", s);   // Error: value moved

    let s2 = String::from("World");
    println!("s2: {}", s2);

    let s3 = take_owner_ship_and_give_back(s2);
    println!("s3 returned: {}", s3);
    
    let (s4, b) = take_owner_ship_and_give_back_multiple(s3);
    println!("s4: {}, b: {}", s4, b);

    use_reference(&s4);
    println!("&s4: {}", s4);
}

fn take_owner_ship(i : i32) {
    println!("i fn: {}", i);
}

fn take_owner_ship_string(s: String) {
    println!("s fn: {}", s);
}

fn take_owner_ship_and_give_back(s2: String) -> String {
    println!("s2 fn take_and_return: {}", s2);
    s2
}

fn take_owner_ship_and_give_back_multiple(s3: String) -> (String, bool) {
    println!("s3 fn return multi: {}", s3);
    (s3, true)
}

fn use_reference(s4 : &String) {
    println!("&s4 fn: {}", s4);
}
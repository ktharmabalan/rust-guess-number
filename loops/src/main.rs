fn main() {
    println!("Result of if expression:");
    // if statement as expression
    let condition = true;
    let if_result = if condition {
        10
    } else {
        20
    };

    println!("{}", if_result);

    println!("Result of break:");
    // Return result of break
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
    assert_eq!(result, 20);

    println!("While loop:");
    // While loop
    let mut count_down = 10;

    while count_down != 0 {
        println!("{}!", count_down);
        count_down = count_down - 1;
    }
    println!("done");

    println!("Loop through collection");
    let a = [5,4,3,2,1,0];
    let mut index = 0;

    while index < 6 {
        println!("For loop {} = {}", index, a[index]);
        index = index + 1;
    }

    println!("Iterate over array");
    for element in a.iter() {
        println!("Element: {}", element);
    }

    println!("Range & Reverse");
    for number in (1..10).rev() {
        println!("Number: {}", number);
    }
}

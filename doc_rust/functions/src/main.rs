fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(42, 'g');

    let result = five();
    println!("fn five() returned {result}");

    let result = plus_one(result);
    println!("result of plus_one() is {result}");
}

// fn another_function() {
//     println!("Another function.");
// }

fn another_function(x: u32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: u32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

// statement   expression
// let y =     { let x = 3; x + 1 expression }; // if you put semicolon at the last expression it turns into a statement and doesn't return a value

fn five() -> i32 {
    // can return early with the return keyword
    return 4 + 1; // this is now a statement
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

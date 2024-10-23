// fn main() {
//     let mut x: i32 = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn main() {
    let x: i32 = 5;

    let x: i32 = x + 1;

    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let x: &str = "changed type to string";

    println!("The value of x is: {x}"); // Shadowing allows transformations of a variable with let. But the variable is still immutable.
                                        // We are effectively creating a new variable with let.
}

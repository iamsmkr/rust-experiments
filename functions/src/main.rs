
fn main() {
    println!("Hello, world!");
    another_function(-5);

    // Curly braces are expressions and they return a value
    let res: () = {

    };
    println!("res = {:?}", res);

    println!("five = {}", five());
}

fn another_function(i: i32) {
    println!("The argument passed = {i}.");
}

fn five() -> i32 {5}

use std::io;
use std::num::Wrapping;

fn main() {
    // Mutable variables
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS = {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = "Shivam";
    println!("Shawdowed x = {x}");

    // Here compiler can't guess the type while parsing hence we must specify the type annotation explicitly
    let _guess: u32 = "42".parse().expect("Not a number!");

    let y = 57u8;
    println!("y = {y}");

    let zero1 = Wrapping(0u8);
    let one1 = Wrapping(1u8);

    println!("Wrapping = {}", (zero1 - one1).0);

    let zero2 = 0u8;
    let one2 = 1u8;

    match zero2.checked_sub(one2) {
        Some(r) => println!("Result = {r}"),
        None => println!("Not an unsigned result"),
    }

    let mut zero3 = String::new();
    io::stdin().read_line(&mut zero3).expect("Failed to read");
    let zero3: u8 = zero3.trim().parse().expect("Failed to parse");

    let mut one3 = String::new();
    io::stdin().read_line(&mut one3).expect("Failed to read");
    let one3: u8 = one3.trim().parse().expect("Failed to parse");

    println!("Wrapping = {}", (zero3 - one3));
}

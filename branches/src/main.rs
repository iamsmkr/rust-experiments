fn main() {
    let num = 6;

    if num > 5 {
        println!("false");
    } else {
        println!("true");
    }

    if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 2, 3 and 4")
    }

    // Returning a value from a loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {result}");

    // Loop lables
    let mut count = 0;
    count = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up count * 2;
            }
            remaining -= 1;
        }
        count += 1;
    };
    println!("count = {count}");

    // For loop
    for n in (1..5).rev() {
        println!("n = {n}")
    }

    fibonacci(10)
}

fn fibonacci(places: u8) {
    let mut i = 0;
    let mut j = 1;
    print!("{i} ");
    print!("{j} ");
    let mut count = 0;

    loop {
        count += 1;
        if count >= places {
            break;
        }
        i = i + j;
        print!("{i} ");
        j = j + i;
        print!("{j} ");
    }

    println!("");
}

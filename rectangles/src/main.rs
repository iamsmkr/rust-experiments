#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width = 30;
    let height = 40;
    println!("Area = {}", area(width, height));

    let rect = (20, 30);
    println!("Area = {}", area1(rect));

    let rect2 = Rectangle {
        width: 100,
        height: 10
    };
    println!("{:#?}", rect2);   // Pretty print
    println!("Area = {}", area2(&rect2));
    println!("{:?}", rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

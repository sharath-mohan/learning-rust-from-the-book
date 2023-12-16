use std::io;
fn main() {
    // wont compile
    // let is_immutable = 5;
    // is_immutable = 6;
    let mut is_mutable = 5;
    println!("mutated value is {}", is_mutable);
    is_mutable = 6;
    println!("mutated value is {}", is_mutable);
    const PI: f64 = 3.14;
    println!("find the area of a circle");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Failed to read");
    let radius: f64 = match radius.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    let area = PI * radius * radius;
    println!("the area of a circle is {}", area);
}

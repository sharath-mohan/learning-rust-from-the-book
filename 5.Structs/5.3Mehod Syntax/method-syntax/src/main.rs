struct Rectangle {
    height: i32,
    width: i32,
}
impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
    fn width(&self)->bool{
        self.width > 0
    }
}
fn main() {
    let rect = Rectangle {
        height: 5,
        width: 6,
    };
    println!("the area is {}", rect.area());
    println!("the react has width ?  {}", rect.width());
}

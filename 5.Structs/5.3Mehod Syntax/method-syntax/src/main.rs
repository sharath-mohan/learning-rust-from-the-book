#[derive(Debug)]
struct Rectangle {
    height: i32,
    width: i32,
}
impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height * self.width > other.height * other.width
    }

    fn square(dim: i32) -> Self {
        Self {
            width: dim,
            height: dim,
        }
    }
}
fn main() {
    let rect = Rectangle {
        height: 5,
        width: 6,
    };
    println!("the area is {}", rect.area());
    println!("the react has width ?  {}", rect.width());
    let rect1 = Rectangle {
        height: 3,
        width: 3,
    };
    let rect2 = Rectangle {
        height: 6,
        width: 6,
    };
    println!("rect1 will fit in rect {}", rect.can_hold(&rect1));
    println!("rect2 will fit in rect {}", rect.can_hold(&rect2));
    println!("create square from rectangle {:?}", Rectangle::square(6));
    println!("{}",Rectangle::area(&rect1));
}

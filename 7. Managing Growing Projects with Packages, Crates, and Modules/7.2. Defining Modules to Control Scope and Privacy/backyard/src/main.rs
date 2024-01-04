use crate::garden::vegetables::Aparagus;
pub mod garden;
fn main() {
    let plant = Aparagus {};
    let apple = garden::fruits::Apples {};
    println!("i am growing {:?}", plant);
    println!("i am growing {:?}", apple);
}

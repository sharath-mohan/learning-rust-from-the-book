struct Dimensions {
    width: i32,
    height: i32,
}
#[derive(Debug)]
struct Point{
    x:i32,
    y:i32,
    z:i32
}
fn main() {
    let width = 2;
    let heigth = 3;
    println!("{}", calc_area(width, heigth));
    let dim: (i32, i32) = (4, 3);
    println!("{}", calc_area_tup(dim));
    let dimensions = Dimensions {
        height: 5,
        width: 6,
    };
    println!("{}", calc_area_struct(&dimensions));
    let point = Point{
        x:1,
        y:1,
        z:1
    };
    println!("{:?}",point);
}

fn calc_area(width: i32, height: i32) -> i32 {
    height * width
}

fn calc_area_tup(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}
fn calc_area_struct(dimensions: &Dimensions) -> i32 {
    dimensions.height * dimensions.width
}

fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_params("sharath");
    let y = {
        let x= 3;
        x+1
    };
    println!("{}",y);
    println!("{}",funtion_with_return_value());
    println!("{}",funtion_with_return_value_and_args(1));
}
fn another_function(){
    println!("another function");
}
fn another_function_with_params(name:&str){
    println!("hello {name}");
}
fn funtion_with_return_value()->i32{
    7
}
fn funtion_with_return_value_and_args(x:i32)->i32{
    x+10
}
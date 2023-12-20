fn main() {
    let number = 3;
    if number > 5 {
        println!("is greater than 5");
    } else {
        println!("is lower than 5");
    }
    let is_zero = 0;
    // will not compile
    // if is_zero{
    //     println!("is zero");
    // }else {
    //     println!("no zero");
    // }
    let result = if is_zero == 0 { true } else { false };
    println!("{}", result);
    loop_function();
}

fn loop_function(){
    loop {
        println!("will keep looping");
    }
}
use std::ops::Add;

fn main() {
    let  my_string = "Hello";
    let mut this_string = String::from("rust");
    //this wont compile as moves out of scope
    // print_about_rust(this_string);
    this_string = this_string.add(" is good");
   this_string = this_string + "and amazing";
    println!("{} , {}", my_string, this_string);
}

fn print_about_rust(this_string:String){
    println!("hello {}", this_string);
}
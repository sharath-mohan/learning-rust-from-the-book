fn main() {
    // integers
    let guess: i32 = "-42".trim().parse().expect("not a number");
    println!("the result of parsing {}", guess);
    // floating point
    let x = 3.0;
    println!("{x}");
    // booleans
    let t = true;
    println!("{t}");
    // char
    let c = 'a';
    println!("{c}");
    // tuples
    let tup = (500, 6.4, false);
    let (d, e, f) = tup;
    println!("{},{},{}", d, e, f);
    println!("{}", tup.0);
    let _unit_tuple = ();
    // arrays
    let months = [
        "Jan", "Feb", "March", "april", "may", "june", "july","august", "sep", "oct", "nov", " dec",
    ];
    println!("{}", months[8]);
    let arr:[i32;3] = [1,2,3];
    println!("{}",arr.len());
    let mut index = String::new();
    std::io::stdin().read_line(&mut index).expect("failed to read file");
    let index:usize = index.trim().parse().expect("not a number");
    let element = arr[index];
    println!("{}",element);
}

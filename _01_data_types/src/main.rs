
fn print_tuple(tuple: (i32, f64, &str)) {
    println!("The value of tuple is: {:?}", tuple);
}


fn fun_with_return() -> (i32, f64, &'static str) {
    let tuple: (i32, f64, &str) = (250, 33.0, "World");
    tuple
}

fn main() {
    println!("Hello, world! Writing the first Program in the Rust Programming Language");

    //  there are two main types of integers in rust 
        // 1. Signed integers: i8, i16, i32, i64, i128, isize
       // 2. Unsigned integers: u8, u16, u32, u64, u128, usize
    let mut num:u8=25;
    println!("The value of num is: {}", num);
    num=30;
    println!("The value of num is: {}", num); 

    //  there are two main types of floating point numbers in rust
        // 1. f32
        // 2. f64
        let mut float_num:f32=25.0;
        println!("The value of float_num is: {}", float_num);
        float_num=30.0;
        println!("The value of float_num is: {}", float_num);


    // string and the &str
        // string -> which is growable and mutable
        // &str -> which is immutable and fixed-size

    let mut string:String=String::from("Column");
    println!("The value of string is: {}", string);
    string.push_str(" Name");
    println!("The value of string is: {}", string);


    let str_slice:&str="This is a string slice";
    println!("The value of str_slice is: {}", str_slice);



    // tuples -> which can hold multiple values of different types
    let tuple:(i32, f64, &str)=(25, 30.0, "Hello");
    println!("The value of tuple is: {:?}", tuple);
    // accessing elements of a tuple can via destructing and via numbering
    println!("The first element of tuple is: {}", tuple.0);
    println!("The second element of tuple is: {}", tuple.1);
    println!("The third element of tuple is: {}", tuple.2);

    //functions
    print_tuple(tuple);

    let returned_tuple: (i32, f64, &str) = fun_with_return();

    print_tuple(returned_tuple);
}


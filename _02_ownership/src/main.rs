
fn get_string() -> String {
    // new_string owns the ownership of the String
    let new_string = String::from("Hello, world!");
    //transferring the ownership
    return new_string;
}

fn send_get_string(received_string:String)->String{
    // received_string owns the ownership of the String
    return received_string;
}


fn calculate_string_length(s: String) -> usize {
    s.len()
}

fn cal_length_return_ownership(s:String)->(usize,String){
    let length = s.len();
    (length,s)
}

fn append_string(s3: &mut String) {
    s3.push_str(" - Appended text");
}

fn return_dangling_ptr() -> &String {
    let s = String::from("Hello, dangling pointer!");
    return &s; // This will cause a compile-time error
}

fn main() {
    println!("Welcome to the Ownership tutorial!");

    let s1 = get_string();
    println!("The value of s1 is: {}", s1);

    let s2 = String::from("Hello, world!");
    println!("The value of s2 is: {}", s2);

    let s3:String=send_get_string(s2);
    println!("The value of s3 is: {}", s3);

    let s4:String=String::from("Hello, Rust!");

    let (length,s4) = cal_length_return_ownership(s4);
    println!("The length of s4 is: {}", length);


    let length = calculate_string_length(s4.clone());
    println!("The length of s4 is: {}", length);
    println!("The value of s4 is: {}", s4);

    let length = calculate_string_length(s4);
    // println!("The value of s4 is: {}", s4);
    println!("The length of s4 is: {}", length);


    let mut s5 = String::from("Hello");
    append_string(&mut s5);
    println!("The value of s5 is: {}", s5);

    // example of the multiple mutable references
    let mut s6 = String::from("Hello");
    let r1=&mut s6;
    r1.push_str("Append-1");
    println!("The value of r1 is: {}", r1);

    let r2=&mut s6;
    r2.push_str("Append-2");
    println!("The value of r2 is: {}", r2);


    // here is the problem -> like in db of perform the multiple write operations, read-write operations ...
    // We cannot have two mutable references to the same data at the same time
            // println!("The value of s6 is: {}", r1);


    // Referencing and the deferencing 
    let mut num=5;
    let y=&mut num;

    println!("The value of y is: {}", y);
    *y=*y+10;
    print!("The value of y is: {}", y);
    println!("The value of num is: {}", num);


    // Dangling Pointers
    let reference_to_nothing = return_dangling_ptr();
    println!("The value of reference_to_nothing is: {}", reference_to_nothing);
    
}
  
use std::io;


fn write_arr(mut arr:[i32;5]){
    arr[0]=10;
    println!("The value of arr in write_arr is: {:?}", arr);
}


fn write_arr_reference(arr: &mut [i32; 5]) {
    arr[0] = 20;
    println!("The value of arr in write_arr_reference is: {:?}", arr);
}

fn write_vrr(v: &mut Vec<&str>) {
    v.push("!");
}   

fn main() {
    println!("Module for the arrays and vectors!");

    //Array 
    // -> is the fixed-size list of elements of the same type.
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of arr is: {:?}", arr);
    println!("length of arr is: {}", arr.len());

    // there is slightly change while passing the aray to the fucntion in comp with the cPP
    // in cpp by default array pass by reference
    // in rust array is passed by value

    write_arr(arr);
    println!("The value of arr after write_arr is: {:?}", arr);


    write_arr_reference(&mut arr);
    println!("The value of arr after write_arr_reference is: {:?}", arr);



    // vectors
    let mut v:Vec<i32> =Vec::new();
    // let mut v2 = vec![1,2,3,4,5];
    // let mut v3 = vec![1;5]; // five elements with value 1
    // let mut v=Vec![1, 2, 3, 4, 5, 5];
    v.push(1);
    v.push(2);
    v.pop();
    v.push(3);
    println!("The value of v is: {:?}", v);
    println!("length of v is: {}", v.len());



    let mut vrr:Vec<&str> = vec!["hello", "world"];
    println!("The value of vrr is: {:?}", vrr);
    println!("length of vrr is: {}", vrr.len());

   write_vrr(&mut vrr);
   println!("The value of vrr after write_vrr is: {:?}", vrr);

   let mut cnt=0;

   loop{
       cnt += 1;
       print!("{} ", cnt);
       if cnt==5 {
           break;
       }    
   }

   while cnt < 10 {
       cnt += 1;
       print!("{} ", cnt);
   }

   for i in 1..=10 {
       print!("{} ", i);
   }

   match cnt {
       1 => print!("One"),
       2 => print!("Two"),
       3 => print!("Three"),
       4 => print!("Four"),
       5 => print!("Five"),
       _ => println!("Something else"),
   }
   print!("Enter some text: ");
   let mut input_text=String::new();
   io::stdin().read_line(&mut input_text).expect("Failed to read line");
   println!("You entered: {}", input_text);

}

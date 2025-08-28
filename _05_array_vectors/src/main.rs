fn main() {
    println!("Inside the array and the vectos block !");

    // Array is the fixed size storing elements
    let a1 = [1, 2, 3, 4, 5];
    println!("Array a1: {:?}", a1);
    println!("Length of a1: {}", a1.len());

    // vectors are the dynamic size storing elements
    let v1 = vec![1, 2, 3, 4, 5];
    println!("Vector v1: {:?}", v1);
    println!("Length of v1: {}", v1.len());

    // Iterators
    // .iter() -> Does not Transfers the Ownership
    // .into_iter() -> does transfer the Ownership
    // .next() -> apply over the iterators

    let a2 = [2, 4, 6, 8, 89, 200];
    for elm in a2.iter() {
        print!("Element: {} ", elm);
    }
    println!("\nLength of a2: {}", a2.len());

    for elm in a2.into_iter() {
        print!("Element: {} ", elm);
    }
    println!("\nLength of a2: {}", a2.len());

    let v2 = vec![2, 4, 6, 8, 89, 200];
    for elm in v2.iter() {
        print!("Element: {} ", elm);
    }
    println!("\nLength of v2: {}", v2.len());

    for elm in v2.into_iter() {
        print!("Element: {} ", elm);
    }
    // next line will show error due to the ownership tranfer ``
    // println!("\nLength of v2: {}", v2.len());
}



struct Rectangle{
    length:usize,
    breadth:usize,
}

impl Rectangle{
    // difference  in the associative functions and the methods are Associative functions can be called without an instance
    // ex: Rectangle::create_rect(10, 20);
    // methods need an instance to be called
    // ex: rect1.change_length(30);
    // Associative functions
    fn create_rect(length: usize, breadth: usize) -> Self {
        Self {
            length,
            breadth,
        }
    }


    // methods

    fn area(&self) -> usize {
        self.length * self.breadth
    }
}

fn change_length(rect: &mut Rectangle, new_length: usize) {
    rect.length = new_length;
}

fn main(){
    println!("Inside the struct block!");

    let mut rect1 = Rectangle::create_rect(10, 20);
    println!("Rectangle 1 - Length: {}, Breadth: {}", rect1.length, rect1.breadth);

    change_length(&mut rect1, 50);
    println!("Rectangle 1 (after change) - Length: {}, Breadth: {}", rect1.length, rect1.breadth);

    let rect2=Rectangle{
        length:30,
        breadth:40,
    };
    println!("Rectangle 2 - Length: {}, Breadth: {}", rect2.length, rect2.breadth);


    println!("Rectangle 2 (area): {}", rect2.area());

}

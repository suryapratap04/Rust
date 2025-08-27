

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

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum Shape{
    Circle(f64), //radius
    Rectangle(f64,f64), //length,breadth
}

impl Shape{
    fn new_circle(radius:f64)->Self{
        Self::Circle(radius)
    }

    fn new_rectangle(length:f64, breadth:f64)->Self{
        Self::Rectangle(length, breadth)
    }


    fn area(&self) -> f64 {
        match self {
            Self::Circle(radius) => std::f64::consts::PI * radius * radius,
            Self::Rectangle(length, breadth) => length * breadth,
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Self::Circle(radius) => 2.0 * std::f64::consts::PI * radius,
            Self::Rectangle(length, breadth) => 2.0 * (length + breadth),
        }
    }
}

fn get_user_number(input:i32)->Option<i32>{
    if input ==1 {
        return Some(1234);
    }else if input==2{
        return Some(5678);
    }
    None
}

fn divide_numbers(num1:i32, num2:i32)->Result<i32, String>{
   if num2 == 0 {
       Err("Division by zero error".into())
   } else {
       Ok(num1 / num2)
   }
}
fn divide_return(num1:i32, num2:i32)->Result<i32, i32>{
   if num2 == 0 {
       Err(0)
   } else {
       Ok(num1 / num2)
   }
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

    println!("Starting the Enum Block");
    let color1 = Color::Red;
    let color2 = Color::Green;
    let color3 = Color::Blue;
    println!("Color1, Color2, Color3 are: {:?}, {:?}, {:?}", color1, color2, color3);

    match color2 {
        Color::Red => println!("Color is Red"),
        Color::Green => println!("Color is Green"),
        Color::Blue => println!("Color is Blue"),
    }

    println!("Starting the Shape Block");
    let circle = Shape::new_circle(5.0);
    let rectangle = Shape::new_rectangle(4.0, 6.0);

    println!("Circle: {:?}", circle);
    println!("Rectangle: {:?}", rectangle);


    println!("Circle (area): {}", circle.area());
    println!("Circle (perimeter): {}", circle.perimeter());

    println!("Rectangle (area): {}", rectangle.area());
    println!("Rectangle (perimeter): {}", rectangle.perimeter());

    println!("Starting of the Optional Enum Block");
    let input1 = 5;
    println!("Input1 is: {} and the Particular number is {:?}",input1, get_user_number(input1));
    match get_user_number(input1) {
        Some(number) => println!("User number is: {}", number),
        None => println!("No user number found"),
    }

    let result = divide_return(10, 0);
    match result {
        Ok(value) => println!("Division result is: {}", value),
        Err(error) => println!("Error occurred: {}", error),
    }

    let result = divide_numbers(10, 0);
    match result {
        Ok(value) => println!("Division result is: {}", value),
        Err(error) => println!("Error occurred: {}", error),
    }
}


// Assignment - 1

// You are building an online learning platform to manage different types of courses. You want to implement a system that can keep track of various courses and provide brief overviews of them.
// Task
// Define a trait named Course, which should have a method get_overview that returns a brief overview of the course as a string.

trait Course{
    fn get_overview(&self) -> String;
}

// Create two structs:

// Workshop with fields: title, instructor, and duration (in hours).
// Seminar with fields: title, speaker, and location.

#[derive(Debug)]
struct Workshop {
    title: String,
    instructor: String,
    duration: u32,
}

#[derive(Debug)]
struct Seminar {
    title: String,
    speaker: String,
    location: String,
}


// Implement the Course trait for both Workshop and Seminar, providing a suitable overview in each case.

impl Course for Workshop {
    fn get_overview(&self) -> String {
        format!("Workshop: {}, Instructor: {}, Duration: {} hours", self.title, self.instructor, self.duration)
    }
}

impl Course for Seminar {
    fn get_overview(&self) -> String {
        format!("Seminar: {}, Speaker: {}, Location: {}", self.title, self.speaker, self.location)
    }
}



// Write a function print_course_overview that takes a generic parameter of any type that implements the Course trait and prints the overview.
fn print_course_overview<T: Course>(course: &T) {
    println!("{}", course.get_overview());
}

fn main() {
    let workshop = Workshop {
        title: String::from("Rust Workshop"),
        instructor: String::from("Alice"),
        duration: 3,
    };

    let seminar = Seminar {
        title: String::from("WebAssembly Seminar"),
        speaker: String::from("Bob"),
        location: String::from("Room 101"),
    };

    print_course_overview(&workshop);
    print_course_overview(&seminar);
}

use std::io;
use rand::prelude::*;

fn main() {
    println!("Writing the Program for the Guessing game!");


    let guess_list=["grapes", "banana", "orange"];
    let mut rng = rand::rng();


    let index=rng.random_range(0..guess_list.len());
    let random_fruit=guess_list[index];

    println!("Random Fruit is = {}", random_fruit);

    loop {
        let mut user_input=String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            let fruit_selected = user_input.trim().to_lowercase();
            println!("User Input is = {}", fruit_selected);

            if random_fruit == fruit_selected {
                println!("You guessed it right!");
                break;
            } else {
                println!("Sorry, try Again");
            }
        },
        Err(e) => {
            println!("Error reading input: {}", e);
        }
    }
    }


}

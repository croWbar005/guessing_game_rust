use std::cmp::Ordering;
use std::io;
use rand::prelude::*;
fn main() {
    loop{
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("error in the way");
    if guess.trim() == "quit".to_string(){
        println!("the user asked to quit!");
        break;
    }
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue
    };
    let random_number = rand::rng().random_range(1..=100);
    println!("the random number is {}",random_number);
    match guess.cmp(&random_number){
        Ordering::Less => println!("guess is less"),
        Ordering::Equal => {
            println!("guess is correct \n the game will end now!");
            break;
        },
        Ordering::Greater => println!("guess is greater")
    }
}
}   

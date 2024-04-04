use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
  println!("Gussing the number");

  let secret_number = rand::thread_rng().gen_range(1..=100);
  // println!("The secret number is {secret_number}");
  

  loop {
    let mut guess = String::new();

    println!("Please input your number here:");
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to parse the input number.");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("Your guess number is {guess}");
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small"),
      Ordering::Greater => println!("Too big"),
      Ordering::Equal => {
        println!("Nice work.");
        break;
      }
    }    
  }
}

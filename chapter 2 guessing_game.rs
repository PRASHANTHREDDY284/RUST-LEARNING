use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main(){
  let random_rumber =  rand::thread_rng().gen_range(1..=100);
  loop{
    println!("Guess the number!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Enter an input");
    let guess: u32 = match guess.trim().parse(){
      Ok(num) => num,
      Err(_) => continue,
    };
    println!("You guessed: {}", guess);
    match guess.cmp(&random_rumber){
      Ordering::Less => println!("Too small"),
      Ordering::Greater => println!("Too big"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}

use std::io::{self, Write}; 
use rand::Rng; 

fn main()
{
    let random_number:i32 = random(1,10);
    let mut guess:String = String::new();
    io::stdout().flush().unwrap(); // we are telling rust to push anything in stdout into the terminal (fixing the whole input before print thing)
    
    loop{
        print!("Make your guess : ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Unknown Error");

        if random_number == guess
        {
            println!("You got it right ! The number was : ", &random_number);
            break;
        } else {
            println!("Sorry man : {} was a wrong guess. Try again !");
        }
    }
}
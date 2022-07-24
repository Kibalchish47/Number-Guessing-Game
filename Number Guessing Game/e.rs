use std::io::{self, Write}; 
use rand::Rng; 

pub fn randomstuff()
{
    let random_number:i32 = rand::thread_rng().gen_range(1..=10);
    println!("$  GAME : GUESS THE NUMBER FROM 1 TO 10. Have fun !");
    println!("$ Debugging : the random number is : {}", &random_number);
    
    loop{
        let mut guess:String = String::new();
        print!("Make your guess : ");

        io::stdout().flush().unwrap(); // we are telling rust to push anything in stdout into the terminal (fixing the whole input before print thing)

        io::stdin()
            .read_line(&mut guess)
            .expect("Unknown Error");

        if random_number.to_string() == guess.trim()
        {
            println!("You got that right, you lucky bastard ! The number indeed was : '{}'", &random_number);
            println!("");
            break;
        } else {
            println!("Sorry man '{}' was a wrong guess. Try again !", &guess.trim());
            println!("");
        }
    }
}

fn main(){
    randomstuff();
}


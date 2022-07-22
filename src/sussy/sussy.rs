use std::io; 

fn main()
{
    print!("Enter your sentence : ");

    let mut hello:String = String::new(); // we define a word to deconstruct

    io::stdin()
        .read_line(&mut hello)
        .expect("Unknown Error");

    for c in hello.chars() 
    {
      println!("{}", &c);
    }
}
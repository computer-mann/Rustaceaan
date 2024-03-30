use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!();
    println!("Guess the number!");
    loop{
    println!("Please input your guess");
    let mut guess=String::new();
    let secret_number=rand::thread_rng().gen_range(1..=100);
    println!("the secret number is {secret_number}");
    io::stdin().read_line(&mut guess)
    .expect("failed to read line");
    let guess:u32=match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue
    };

    match guess.cmp(&secret_number) {
        Ordering::Less=> println!("too small!"),
        Ordering::Equal=>{ println!("guessed right"); break },
        Ordering::Greater=> println!("too big")
    }
    
    println!("you guessed: {guess}");
    println!();
    }
}

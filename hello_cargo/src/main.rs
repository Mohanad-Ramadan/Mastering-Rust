use std::io;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    
    println!("the secret number is : {random_number}");
    println!("please enter your guss");

    let mut guss = String::new();

    io::stdin()
        .read_line(&mut guss)
        .expect("Fail to read line");

    println!("You guessed: {guss}");
}

use core::num;
use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let random_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("please enter your guss");
        
        let mut guss = String::new();
        
        io::stdin()
        .read_line(&mut guss)
        .expect("Fail to read line");
        
        let guss: i32 = match guss.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guss.cmp(&random_number) {
            Ordering::Equal => {
                println!("You gussed it right, You win!!");
                break;
            }
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("Too samll!")
        }
    }
    
}

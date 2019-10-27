extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    let mut input = String::new();
    
    println!("What input of di(c)e whod you like to roll?");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            if input.trim() == "20" {
                d20()
            } else if input.trim() == "6" {
                d6()
            }else {
                d66()
            }
        },
        Err(e) => println!("{}", e)
    }
}

fn d20() {
    let x = rand::thread_rng().gen_range(1, 20);
    println!("{}", x);
}

fn d6() {
    let x = rand::thread_rng().gen_range(1, 6);
    println!("{}", x);
}

fn d66() {
    let x = rand::thread_rng().gen_range(1, 6);
    let y = rand::thread_rng().gen_range(1, 6);
    println!("{}{}", x,y);
}


extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    let mut input = String::new();
    
    println!("What type of di(c)e whod you like to roll?");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            if input.replace("d"," ").trim() == "20" {
                d20()
            } else if input.replace("d"," ").trim() == "6" {
                d6()
            }else if input.replace("d"," ").trim() == "66" {
                d66()
            } else if input.replace("d"," ").trim() == "4" {
                d4()
            } else if input.replace("d"," ").trim() == "8" {
                d8()
            } else if input.replace("d"," ").trim() == "10" {
                d10()
            } else if input.replace("d"," ").trim() == "12" {
                d12()
            } else {
                d00()
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
    let z = x+y;
    println!("{}", z);
}

fn d4() { 
    let x = rand::thread_rng().gen_range(1, 4);
    println!("{}", x);
}

fn d8() { 
    let x = rand::thread_rng().gen_range(1, 8);
    println!("{}", x);
}

fn d10() { 
    let x = rand::thread_rng().gen_range(1, 10);
    println!("{}", x);
}

fn d12() { 
    let x = rand::thread_rng().gen_range(1, 12);
    println!("{}", x);
}

fn d00() { 
    let x = rand::thread_rng().gen_range(1, 100);
    println!("{}", x);
}

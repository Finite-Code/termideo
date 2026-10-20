use std::io;

pub mod vid;
pub mod img;

use crate::img::image;
use crate::vid::video;

fn main(){
    println!("Welcome to termideo, get started by selecting:");
    println!("   1. Image");
    println!("   2. Video");

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read!");

    let slctn: i32 = match selection.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Ehh: Did you enter a valid number?");
            return;
        }
    };

    if slctn==1 {
        println!("Starting in Image mode...");
        image::image();
    }
    else if slctn==2{
        println!("Starting in Video mode...");
        video::video();
    }
    else{
        println!("Invalid Selection!");
    }
}
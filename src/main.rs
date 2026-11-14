use std::io;

pub mod vid;
pub mod img;

use crate::img::image;
use crate::vid::video;

fn main(){

    let mut path = String::new();

    // path = "screw you Ananya".to_string(); just to check if the parser is working, hehe

    if(path.trim().is_empty()){
        println!("You didn't submit a path, paste one in, or press Ctrl+C to exit.");
    }

    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read a path!");

    path = path.trim().to_string();
}
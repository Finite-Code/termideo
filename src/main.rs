use std::io;

pub mod vid;
pub mod img;

use crate::img::image;
use crate::vid::video;

fn main() {
    let mut path = String::new();

    println!("You sir, didn't submit a path, paste one in, or press Ctrl+C to exit.");
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read a path!");

    let path = path.trim();

    if path.ends_with(".mp4") || path.ends_with(".mov") || path.ends_with(".gif") {
        println!("going for the video mode huh? cool.");
    } 
    else if path.ends_with(".png") || path.ends_with(".jpg") || path.ends_with(".jpeg") {
        println!("going for the image stuff? great choice, but video mode's a flex of power!");
    } 
    else {
        println!("something's not right... check the supported formats, path");
    }
}
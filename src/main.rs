use std::io;
use std::env;

// the modularized components
pub mod vid;
pub mod img;
use crate::img::image;
use crate::vid::video;
use crate::video::video;
use crate::image::image;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = String::new();

    if args.len() > 1{
        path = args[1].clone();
    } else {
        println!("You sir, didn't submit a path, paste one in, or press Ctrl+C to exit.");
        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read a path!");
    }

    let path = path.trim();

    if path.ends_with(".mp4") || path.ends_with(".mov") || path.ends_with(".gif") {
        println!("Video playback initialized"); // this is allowed because video will wipe the terminal clean

        if let Err(e) = video(path){
            println!("Oops! Something has went wrong. Details here: {}", e)
        }
    } 
    else if path.ends_with(".png") || path.ends_with(".jpg") || path.ends_with(".jpeg") {
        // println!("Video playback initialized"); won't clutter the terminal with such text.

        if let Err(e) = image(path){
            println!("Oops! Something has went wrong. Details here: {}", e)
        }
    } 
    else {
        println!("something's not right... check the supported formats, path");
    }
}
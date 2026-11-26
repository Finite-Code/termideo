use std::io;
use crossterm::{execute, cursor::{Show, MoveToNextLine}, terminal::{Clear, ClearType}};
use std::io::{stdout, Write};
use std::env;


// the modularized components
pub mod vid;
pub mod img;
use crate::img::image::image;
use crate::vid::video::video;

fn main() {
    letme_exit();

    let args: Vec<String> = env::args().collect();
    let mut path = String::new();

    if args.len() > 1{
        let arg = &args[1];

        path = args[1].clone();

        if arg=="-h" || arg == "--help" || arg == "help"{
            help();
            return();
        }
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

fn help(){
    println!("Termideo: ASCII Video, now over the terminal with Image Support!");
    // println!("          with GPU acceleration support for video."); some day
    println!("USAGE: ");
    println!("      termideo <PATH>         Displays any supported image or video file.");
    println!("      termideo -h, --help     Show this message");
    println!("SUPPORTED FORMATS: ");
    println!("      Image: .png .jpg .jpeg .tiff .bmp .ico .avif");
    println!("      Video: .mp4 .mov .gif")
}

fn letme_exit() {
    ctrlc::set_handler(move || {
            // display cursor before we leave
            let _ = execute!(stdout(), MoveToNextLine(1), Clear(ClearType::CurrentLine), Show);

            println!("Performing a clean exit...");
            let _ = stdout().flush();

            println!("Done");
            std::process::exit(0);
        }).expect("Error exiting cleanly!");
}
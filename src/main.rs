use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use image::ImageReader;
use image::GenericImageView;

use std::io::{stdout, Write};
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let buffer = "bad.jpg";

    let img = ImageReader::open(buffer)?.decode()?;

    let (sizeX, sizeY) = img.dimensions();

    println!("sizeX: {sizeX}");
    println!("sizeY: {sizeY}");

    Ok(())
}
use image::ImageReader;
use image::GenericImageView;

use crossterm::{execute, cursor::{Hide, Show, MoveTo}, terminal::{Clear, ClearType, size}};

use std::io::{stdout, Write};

pub fn image(buffer: &str) -> Result<(), Box<dyn std::error::Error>> {

    // let buffer = "img/bad.jpg"; don't need this anymore
    let density = " .,:;i1tfLCG08#@";
	let chars: Vec<char> = density.chars().collect();
    let img = ImageReader::open(buffer)?.decode()?;

    let img = ImageReader::open(buffer)?.decode()?;
    let (term_w, term_h) = size()?;

    let tgt_h = (term_h - 1) as u32;
    let tgt_w = term_w as u32;
    let reimg = img.resize_exact(tgt_w, tgt_h, image::imageops::FilterType::Lanczos3);
    let (wdh, hgh) = img.dimensions();

    execute!(stdout(), Hide, Clear(ClearType::All), MoveTo(0, 0))?;

    let mut framebuffer = String::new();

    for y in (0..hgh).step_by(2){
        let mut row_str = String::new();

        for x in 0..wdh{
            let pix = img.get_pixel(x, y);

            let r = pix[0] as f32;
            let g = pix[1] as f32;
            let b = pix[2] as f32;

            // Internet formula: high green cause we are sensitive to it
            let mut luma = (r*0.2126) + (g*0.7152) + (b*0.0722);

            if luma < 15.0 {luma=0.00;}
			if luma >240.0 {luma=255.0;}

            let index = ((luma/255.0) * (chars.len() - 1) as f32) as usize;
            row_str.push(chars[index]);
        }

        framebuffer.push_str(&row_str);
        framebuffer.push('\n');
    }

    print!("{}", framebuffer);
    stdout().flush()?;
    execute!(stdout(), Show)?;

    Ok(())
}
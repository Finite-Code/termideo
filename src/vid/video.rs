use opencv::videoio;
use opencv::prelude::*;
use opencv::imgproc;
use opencv::core::Size;

use crossterm;
use crossterm::{execute, cursor::MoveTo};

use std::io::stdout;
use std::io::Write;
use std::{thread, time::Duration};
use std::time::Instant;

pub fn video() -> Result<(), Box<dyn std::error::Error>> {

	let v_path = "vid/bad.mp4"; // hardcoded bad apple video for now...
	let density = " .,:;i1tfLCG08#@";
	let chars: Vec<char> = density.chars().collect();

	let mut mat = Mat::default();
	let mut remat = Mat::default(); // another container!
	let mut vid = videoio::VideoCapture::from_file(v_path, videoio::CAP_ANY)?;

	let mut framebuffer = String::new();

	let fps = vid.get(videoio::CAP_PROP_FPS)?;
	let frametime = (1000.0/fps) as u64;

	let tgt_f_time = Duration::from_millis((1000.0/fps) as u64);

	println!("Starting video processing...");

	loop {
		let start = std::time::Instant::now();

		let (term_w, term_h) = crossterm::terminal::size()?; // Keep checking the size
		
		let success = vid.read(&mut mat)?;

		imgproc::resize(
			&mat, &mut remat,
			Size::new(term_w as i32, (term_h - 1) as i32),
			0.0, 0.0,
			imgproc::INTER_LINEAR
		)?;

		if !success {
			println!("Video Ended!");
			break;
		}

		let wdh = remat.cols(); // define width
		let hgh = remat.rows(); //  and   height

		execute!(stdout(), MoveTo(0,0))?;

    	let mut framebuffer = String::new();

		for y in (0..hgh){ // DON'T stepby(2) anymore cuz imgproc's got it
        	for x in 0..wdh{

				let pix = remat.at_2d::<opencv::core::Vec3b>(y, x)?;

            	let b = pix[0] as f32;
            	let g = pix[1] as f32;
            	let r = pix[2] as f32;

            	// Internet formula: high green cause we are sensitive to it
				// Also: opencv uses BGR
            	let luma = (b*0.0722) + (g*0.7152) + (r*0.2126);

            	let index = ((luma/255.0) * (chars.len() - 1) as f32) as usize;

            	let ascii_char = chars[index];
            	framebuffer.push(ascii_char);
        	}
			framebuffer.push('\n');
    	}
		println!("{}", framebuffer);
		stdout().flush()?;

		if let Some(remaining) = tgt_f_time.checked_sub(start.elapsed()){
			thread::sleep(remaining);
		}
	}

	Ok(())
}
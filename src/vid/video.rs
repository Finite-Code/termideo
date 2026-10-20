use opencv::videoio;
use opencv::prelude::*;

pub fn video() -> Result<(), Box<dyn std::error::Error>> {

	let _v_path = "vid/bad.mp4"; // hardcoded bad apple video for now...
	let _density = " .:-=*#%@";

	let mut mat = Mat::default();
	let mut vid = videoio::VideoCapture::from_file(_v_path, videoio::CAP_ANY)?;

	println!("Starting video processing...");

	loop {
		let success = vid.read(&mut mat)?;

		if !success{
			println!("failed to grab a frame");
			break;
		}

		println!("found a frameeeeeeeee")
	}

	Ok(())
}
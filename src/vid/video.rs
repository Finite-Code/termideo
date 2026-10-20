use opencv::videoio;
use opencv::prelude::*;

pub fn video() -> Result<(), Box<dyn std::error::Error>> {

	let v_path = "vid/bad.mp4"; // hardcoded bad apple video for now...
	let density = " .:-=*#%@";

	let mut mat = Mat::default();
	let mut vid = videoio::VideoCapture::from_file(v_path, videoio::CAP_ANY)?;

	println!("Starting video processing...");

	loop {
		let success = vid.read(&mut mat)?;


		let wdh = mat.cols(); // define width
		let hgh = mat.rows(); //  and   height

		for y in (0..hgh).step_by(2){
        	let mut row_str = String::new();

        	for x in 0..wdh{

				let pix = mat.at_2d::<opencv::core::Vec3b>(y, x)?;

            	let b = pix[0] as f32;
            	let g = pix[1] as f32;
            	let r = pix[2] as f32;

            	// Internet formula: high green cause we are sensitive to it
				// Also: opencv uses BGR
            	let luma = (b*0.0722) + (g*0.7152) + (r*0.2126);

            	let index = ((luma/255.0) * (density.len() - 1) as f32) as usize;

            	let ascii_char = density.chars().nth(index).unwrap();
            	row_str.push(ascii_char);
        	}

        	println!("{}", row_str);
    	}
		
		if !success {
			println!("Video Ended!");
			break;
		}
	}

	Ok(())
}
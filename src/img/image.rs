use image::ImageReader;
use image::GenericImageView;

pub fn image() -> Result<(), Box<dyn std::error::Error>> {

    let buffer = "img/bad.jpg";
    let density = " .,:;i1tfLCG08#@";
	let chars: Vec<char> = density.chars().collect();
    let img = ImageReader::open(buffer)?.decode()?;

    let (wdh, hgh) = img.dimensions();

    for y in (0..hgh).step_by(2){
        let mut row_str = String::new();

        for x in 0..wdh{
            let pix = img.get_pixel(x, y);

            let r = pix[0] as f32;
            let g = pix[1] as f32;
            let b = pix[2] as f32;

            // Internet formula: high green cause we are sensitive to it
            let luma = (r*0.2126) + (g*0.7152) + (b*0.0722);

            let index = ((luma/255.0) * (density.len() - 1) as f32) as usize;

            let ascii_char = chars[index];
            row_str.push(ascii_char);
        }

        println!("{}", row_str);
    }

    Ok(())
}
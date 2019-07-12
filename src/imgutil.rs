extern crate image;
use image::{ImageBuffer, Rgb};

// --- types (best place for this?)
pub type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;


// --- functions
pub fn flip_img_y(img: Image) -> Image {
	let mut result: Image = ImageBuffer::new(img.width(), img.height());
	for y in 0..img.height() {
		let op_y = img.height() - (y + 1);
		for x in 0..img.width() {
			result.get_pixel_mut(x as u32, op_y as u32).data = img.get_pixel(x as u32, y as u32).data;
		}
	}
	result
}
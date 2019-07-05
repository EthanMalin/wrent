// --- usings
extern crate image;

mod geometry;
mod draw;
mod model;

use image::{ImageBuffer, Rgb};
use model::Model;

// --- typedefs
type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

// --- consts
const WIDTH: u32 = 1400;
const HEIGHT: u32 = 1400;

#[cfg(target_os = "macos")]
const IMG_PATH: &str = "/Users/ethanmalin/Desktop/projects/wrent/img/generated.png";
#[cfg(target_os = "macos")]
const MODEL_PATH: &str = "/Users/ethanmalin/Desktop/projects/wrent/obj/head.obj";

#[cfg(taget_os = "windows")]
const IMG_PATH: &str = "C:\\Users\\Ethan\\dev\\projects\\wrent\\img\\generated.png";
#[cfg(taget_os = "windows")]
const MODEL_PATH: &str = "C:\\Users\\Ethan\\dev\\projects\\wrent\\obj\\head.obj";


// --- code

fn flip_img_y(img: Image) -> Image {
	let mut result: Image = ImageBuffer::new(img.width(), img.height());
	for y in 0..img.height() {
		let op_y = img.height() - (y + 1);
		for x in 0..img.width() {
			result.get_pixel_mut(x as u32, op_y as u32).data = img.get_pixel(x as u32, y as u32).data;
		}
	}
	result
}
fn main() {

	// draw target
	// let mut image: Image = ImageBuffer::new(WIDTH+1, HEIGHT+1);

	// let model = Model::new(MODEL_PATH).unwrap();
	// let white = [255, 255, 255];
	// let red = [255, 0, 0];
	// draw::model_filled(&model, &mut image, red);
	// draw::model_wireframe(&model, &mut image, white);

	//image.reverse(); is not *just* a flip over horizontal axis
	let mut image: Image = ImageBuffer::new(WIDTH+1, HEIGHT+1);
	image = flip_img_y(image);
	image.save(IMG_PATH).unwrap();
}
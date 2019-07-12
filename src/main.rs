// --- usings
extern crate image;

mod geometry;
mod draw;
mod model;
mod imgutil;

use image::ImageBuffer;
use model::Model;

// --- typedefs
type Image = imgutil::Image;	// typedef probably shouldn't be in utils

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
fn main() {
	let mut image: Image = ImageBuffer::new(WIDTH+1, HEIGHT+1);
	image = draw_model(MODEL_PATH, image);
	image = imgutil::flip_img_y(image);
	image.save(IMG_PATH).unwrap();
}

fn draw_model(model_path: &str, mut image: Image) -> Image {
	let model = Model::new(model_path).unwrap();
	let white = [255, 255, 255];
	let red = [255, 0, 0];
	draw::model::model_filled(&model, &mut image, red);
	draw::model::model_wireframe(&model, &mut image, white);
	image
}
// --- usings
extern crate image;
use image::{ImageBuffer, Rgb};

mod draw;

mod geometry;
use geometry::Vec2;

mod model;
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


// --- code
fn main() {

	// draw target
	let mut image: Image = ImageBuffer::new(WIDTH+1, HEIGHT+1);

	let model = Model::new(MODEL_PATH).unwrap();
	let red = [255, 255, 255];
	draw::model_filled(&model, &mut image, red);
	//draw::model_wireframe(&model, &mut image, [255,255,255]);

	// let a: [Vec2<i32>; 3] = [Vec2::new(10, 70), Vec2::new(70, 80), Vec2::new(50, 160)];
	// let b: [Vec2<i32>; 3] = [Vec2::new(150, 40), Vec2::new(250, 160), Vec2::new(370, 80)];
	// let c: [Vec2<i32>; 3] = [Vec2::new(600, 500), Vec2::new(500, 600), Vec2::new(400, 550)];


	// image.reverse(); is not *just* a flip over horizontal axis
	image.save(IMG_PATH).unwrap();
}
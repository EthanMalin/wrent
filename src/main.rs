// --- usings
extern crate image;
use image::{ImageBuffer, Rgb};

mod geometry;
mod model;

// --- typedefs
type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

// --- consts
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
#[cfg(target_os = "macos")]
const IMG_PATH: &str = "/Users/ethanmalin/Desktop/projects/wrent/img/generated.png";
#[cfg(target_os = "macos")]
const MODEL_PATH: &str = "/Users/ethanmalin/Desktop/projects/wrent/obj/head.obj";

#[cfg(taget_os = "windows")]
const IMG_PATH: &str = "C:\\Users\\Ethan\\dev\\projects\\wrent\\img\\generated.png";


// --- code
fn main() {

	let model = model::Model::new(MODEL_PATH).unwrap();
	let mut image: Image = ImageBuffer::new(WIDTH+1, HEIGHT+1);

	for face in model.faces.iter() {
		let v0 = model.vert(face.x as usize);
		let v1 = model.vert(face.y as usize);
		let v2 = model.vert(face.z as usize);

		// coordinates are normalized to [-1, 1], so we need to de-normalize them to fit them on screen
		let v0x = ( ((v0.x + 1f64) * (WIDTH as f64)) / 2f64 ) as i32;
		let v0y = ( ((v0.y + 1f64) * (WIDTH as f64)) / 2f64 ) as i32;

		let v1x = ( ((v1.x + 1f64) * (WIDTH as f64)) / 2f64 ) as i32;
		let v1y = ( ((v1.y + 1f64) * (WIDTH as f64)) / 2f64 ) as i32;

		let v2x = ( ((v2.x + 1f64) * (WIDTH as f64)) / 2f64 ) as i32;
		let v2y = ( ((v2.y + 1f64) * (WIDTH as f64)) / 2f64 ) as i32;

		line(v0x, v0y, v1x, v1y, &mut image, [255, 255, 255]);
		line(v0x, v0y, v2x, v2y, &mut image, [255, 255, 255]);
		line(v2x, v2y, v1x, v1y, &mut image, [255, 255, 255]);
	}

	image.save(IMG_PATH).unwrap();
}

// bresenham's
fn line(mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, img: &mut Image, color: [u8; 3]) {
	let mut steep = false;
	if (x1-x2).abs() < (y1-y2).abs() {
		let mut temp = y1;
		y1 = x1;
		x1 = temp;

		temp = y2;
		y2 = x2;
		x2 = temp;
		steep = true;
	}

	if x1 > x2 {
		let mut temp = x2;
		x2 = x1;
		x1 = temp;

		temp = y2;
		y2 = y1;
		y1 = temp;
	}

	let dx = x2 - x1;
	let dy = y2 - y1;
	let derror2 = 2*dy.abs();
	let mut error2 = 0;
	let mut y = y1;

	if steep {
		for x in x1..(x2+1) {
			img.get_pixel_mut(y as u32, x as u32).data = color;
			error2 += derror2;
			if error2 > dx {
				if y2 > y1 { y += 1; } else { y -= 1; }
				error2 -= dx*2;
			}
		}
	} else {
		for x in x1..(x2+1) {
			img.get_pixel_mut(x as u32, y as u32).data = color;
			error2 += derror2;
			if error2 > dx {
				if y2 > y1 { y += 1; } else { y -= 1; }
				error2 -= dx*2;
			}
		}		
	}
}
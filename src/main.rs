extern crate image;

use image::{ImageBuffer, Rgb};

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

#[cfg(target_os = "macos")]
const IMG_PATH: &str = "/Users/ethanmalin/Desktop/projects/wrent/img/generated.png";
#[cfg(taget_os = "windows")]
const IMG_PATH: &str = "C:\\Users\\Ethan\\dev\\projects\\wrent\\img\\generated.png";

fn main() {
		let mut image: Image = ImageBuffer::new(WIDTH, HEIGHT);
		line(1, 1, 94, 94, &mut image, [255, 255, 255]);
		line(1, 94, 93, 2, &mut image, [255, 0, 0]);
		line(27, 13, 90, 11, &mut image, [255, 0, 0]);
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

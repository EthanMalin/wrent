extern crate image;
use image::{ImageBuffer, Rgb};
use crate::geometry::{Vec2, Vec3};
use crate::model::Model;

// --- typedefs
type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

// bresenham's
pub fn line_points(mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, img: &mut Image, color: [u8; 3]) {
	let mut steep = false;

	// if the line is too steep, transpose
	if (x1-x2).abs() < (y1-y2).abs() {
		std::mem::swap(&mut x1, &mut y1);
		std::mem::swap(&mut x2, &mut y2);
		steep = true;
	}

	// force the leftmost point to be x1, y1 
	if x1 > x2 {
		std::mem::swap(&mut x1, &mut x2);
		std::mem::swap(&mut y1, &mut y2);
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

pub fn line_vecs(v0: &Vec2<i32>, v1: &Vec2<i32>, img: &mut Image, color: [u8; 3]) {
	line_points(v0.x, v0.y, v1.x, v1.y, img, color);
}

pub fn triangle_wireframe(v0: &Vec2<i32>, v1: &Vec2<i32>, v2: &Vec2<i32>, img: &mut Image, color: [u8; 3]) {
	line_vecs(v0, v1, img, color);
	line_vecs(v0, v2, img, color);
	line_vecs(v1, v2, img, color);
}

pub fn triangle_filled(v0: &mut Vec2<i32>, v1: &mut Vec2<i32>, v2: &mut Vec2<i32>, img: &mut Image, color: [u8; 3]) {
	// sort the vertices by increasing y value
	if v0.y > v1.y { std::mem::swap(v0, v1); }
	if v0.y > v2.y { std::mem::swap(v0, v2); }
	if v1.y > v2.y { std::mem::swap(v1, v2); }

	// for i in 
}

pub fn model_wireframe(mdl: &Model, mut img: &mut Image, color: [u8; 3]) {
	for face in mdl.faces.iter() {
		let v0 = mdl.vert(face.x as usize);
		let v1 = mdl.vert(face.y as usize);
		let v2 = mdl.vert(face.z as usize);

		// coordinates are normalized to [-1, 1], so we need to de-normalize them to fit them on screen
		let v0x = ( ((v0.x + 1f64) * ( (img.width()-1) as f64)) / 2f64 ) as i32;
		let v0y = ( ((v0.y + 1f64) * ( (img.height()-1) as f64)) / 2f64 ) as i32;

		let v1x = ( ((v1.x + 1f64) * ( (img.width()-1) as f64)) / 2f64 ) as i32;
		let v1y = ( ((v1.y + 1f64) * ( (img.height()-1) as f64)) / 2f64 ) as i32;

		let v2x = ( ((v2.x + 1f64) * ( (img.width()-1) as f64)) / 2f64 ) as i32;
		let v2y = ( ((v2.y + 1f64) * ( (img.height()-1) as f64)) / 2f64 ) as i32;

		line_points(v0x, v0y, v1x, v1y, &mut img, [255, 255, 255]);
		line_points(v0x, v0y, v2x, v2y, &mut img, [255, 255, 255]);
		line_points(v2x, v2y, v1x, v1y, &mut img, [255, 255, 255]);
	}
}
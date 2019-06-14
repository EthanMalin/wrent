extern crate image;
use image::{ImageBuffer, Rgb};
use crate::geometry;
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

pub fn triangle_filled(pts: [&Vec2<i32>; 3], img: &mut Image, color: [u8; 3]) {
	
	// find bounding box
	let clamp = Vec2::new((img.width() as i32) - 1, (img.height() as i32) - 1);
	let mut bbox_max = Vec2::new(pts[0].x, pts[0].y);
	let mut bbox_min = Vec2::new(pts[0].x, pts[0].y);

	let mid_y: i32 = std::cmp::min(pts[2].y, std::cmp::max(pts[0].y, pts[1].y));

	for k in 1..3 {
		bbox_min.x = std::cmp::min::<i32>(bbox_min.x, pts[k].x);
		bbox_min.y = std::cmp::min::<i32>(bbox_min.y, pts[k].y);
 
		bbox_max.x = std::cmp::max::<i32>(bbox_max.x, pts[k].x);
		bbox_max.y = std::cmp::max::<i32>(bbox_max.y, pts[k].y);
	}


	for x in bbox_min.x..(bbox_max.x + 1) {
		for y in bbox_min.y..(bbox_max.y + 1) {
			let bc_screen = geometry::barycentric(pts, &Vec2::new(x, y));

			if y > mid_y {
				if bc_screen.x < 0.0 || bc_screen.y < 0.0 { continue; }
			} else {
				if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 { continue; }
			}
			img.get_pixel_mut(x as u32, y as u32).data = color;
		}
	}
	triangle_wireframe(pts[0], pts[1], pts[2], img, color);

}

pub fn model_wireframe(mdl: &Model, img: &mut Image, color: [u8; 3]) {
	for face in mdl.faces.iter() {
		let v0 = mdl.vert(face.x as usize);
		let v1 = mdl.vert(face.y as usize);
		let v2 = mdl.vert(face.z as usize);

		// coordinates are normalized to [-1, 1], so we need to de-normalize them to fit them on screen
		let v0x = ( ((v0.x + 1.0) * ( (img.width()-1) as f64)) / 2.0 ) as i32;
		let v0y = ( ((v0.y + 1.0) * ( (img.height()-1) as f64)) / 2.0 ) as i32;

		let v1x = ( ((v1.x + 1.0) * ( (img.width()-1) as f64)) / 2.0 ) as i32;
		let v1y = ( ((v1.y + 1.0) * ( (img.height()-1) as f64)) / 2.0 ) as i32;

		let v2x = ( ((v2.x + 1.0) * ( (img.width()-1) as f64)) / 2.0 ) as i32;
		let v2y = ( ((v2.y + 1.0) * ( (img.height()-1) as f64)) / 2.0 ) as i32;

		line_points(v0x, v0y, v1x, v1y, img, [255, 255, 255]);
		line_points(v0x, v0y, v2x, v2y, img, [255, 255, 255]);
		line_points(v2x, v2y, v1x, v1y, img, [255, 255, 255]);
	}
}


pub fn model_filled(mdl: &Model, img: &mut Image, color: [u8; 3]) {
	let light_dir = Vec3::new(0.0, 0.0, -1.0);
	let mut draw = true;
	for face in mdl.faces.iter() {
		let w0 = mdl.vert(face.x as usize);
		let w1 = mdl.vert(face.y as usize);
		let w2 = mdl.vert(face.z as usize);

		// coordinates are normalized to [-1, 1], so we need to de-normalize them to fit them on screen
		let s0x = ( ((w0.x + 1.0) * ( (img.width()-1) as f64)) / 2.0 ) as i32;
		let s0y = ( ((w0.y + 1.0) * ( (img.height()-1) as f64)) / 2.0 ) as i32;

		let s1x = ( ((w1.x + 1.0) * ( (img.width()-1) as f64)) / 2.0 ) as i32;
		let s1y = ( ((w1.y + 1.0) * ( (img.height()-1) as f64)) / 2.0 ) as i32;

		let s2x = ( ((w2.x + 1.0) * ( (img.width()-1) as f64)) / 2.0 ) as i32;
		let s2y = ( ((w2.y + 1.0) * ( (img.height()-1) as f64)) / 2.0 ) as i32;

		let mut normal = Vec3::cross( &(w2-w0) , &(w1-w0) );
		normal.normalize();
		let intensity = Vec3::dot(&normal, &light_dir);

		if intensity > 0.0 && draw {
			let v0 = Vec2::new(s0x, s0y);
			let v1 = Vec2::new(s1x, s1y);
			let v2 = Vec2::new(s2x, s2y);

			let adjusted_color = [ (intensity*color[0] as f64) as u8, (intensity*color[1] as f64) as u8, (intensity*color[2] as f64) as u8];

			triangle_filled([&v0, &v1, &v2], img, adjusted_color);
			draw = false;
			continue;
		}
		draw = true;


	}
}
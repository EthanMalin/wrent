extern crate image;
use crate::geometry;
use crate::geometry::{Vec2, Vec3};
use crate::imgutil::Image;

// --- types
pub type ColorRGB = [u8; 3]; 


// --- functions
// bresenham's
pub fn line_points(mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, img: &mut Image, color: ColorRGB) {
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

pub fn line_vecs(v0: &Vec2<i32>, v1: &Vec2<i32>, img: &mut Image, color: ColorRGB) {
	line_points(v0.x, v0.y, v1.x, v1.y, img, color);
}

pub fn line_vec3(v0: &Vec3<i32>, v1: &Vec3<i32>, img: &mut Image, color: ColorRGB) {
    line_points(v0.x, v0.y, v1.x, v1.y, img, color);
}

pub fn triangle_wireframe(v0: &Vec2<i32>, v1: &Vec2<i32>, v2: &Vec2<i32>, img: &mut Image, color: ColorRGB) {
	line_vecs(v0, v1, img, color);
	line_vecs(v0, v2, img, color);
	line_vecs(v1, v2, img, color);
}

pub fn triangle_filled(pts: [&Vec2<i32>; 3], img: &mut Image, color: ColorRGB) {
	
	// find bounding box
	let clamp = Vec2::new((img.width() as i32) - 1, (img.height() as i32) - 1);
	let mut bbox_max = Vec2::new(0, 0);
	let mut bbox_min = Vec2::new((img.width() as i32) - 1, (img.height() as i32) - 1);

	for k in 0..3 {
		bbox_min.x = std::cmp::max::<i32>(0, std::cmp::min::<i32>(bbox_min.x, pts[k].x));
		bbox_min.y = std::cmp::max::<i32>(0, std::cmp::min::<i32>(bbox_min.y, pts[k].y));
 
		bbox_max.x = std::cmp::min::<i32>(clamp.x, std::cmp::max::<i32>(bbox_max.x, pts[k].x));
		bbox_max.y = std::cmp::min::<i32>(clamp.y, std::cmp::max::<i32>(bbox_max.y, pts[k].y));
	}

	for x in bbox_min.x..(bbox_max.x + 1) {
		for y in bbox_min.y..(bbox_max.y + 1) {
			let bc_screen = geometry::barycentric(pts, &Vec2::new(x, y));
			if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 { continue; }
			img.get_pixel_mut(x as u32, y as u32).data = color;
		}
	}
}
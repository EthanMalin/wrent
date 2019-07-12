use crate::imgutil;
use crate::geometry::{Vec2, Vec3};
use crate::model::Model;
use crate::draw::core;

// --- typedefs
type Image = imgutil::Image;	// somewhere common to put this typedef ?

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

		core::line_points(v0x, v0y, v1x, v1y, img, [color[0], color[1], color[2]]);
		core::line_points(v0x, v0y, v2x, v2y, img, [color[0], color[1], color[2]]);
		core::line_points(v2x, v2y, v1x, v1y, img, [color[0], color[1], color[2]]);
	}
}


pub fn model_filled(mdl: &Model, img: &mut Image, color: [u8; 3]) {
	let light_dir = Vec3::new(0.0, 0.0, -1.0);
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

		let mut normal = Vec3::cross( &(v2-v0) , &(v1-v0) );
		normal.normalize();
		let intensity = Vec3::dot(&normal, &light_dir);

		if intensity > 0.0 {
			let v0 = Vec2::new(v0x, v0y);
			let v1 = Vec2::new(v1x, v1y);
			let v2 = Vec2::new(v2x, v2y);

			let adjusted_color = [ (intensity*color[0] as f64) as u8, (intensity*color[1] as f64) as u8, (intensity*color[2] as f64) as u8];

			core::triangle_filled([&v0, &v1, &v2], img, adjusted_color);
			continue;
		}
	}
}
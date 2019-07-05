extern crate num_traits;

use std::ops::{Add, Sub, Mul, Div};
use num_traits::float::Float;
// --- structs : Vec2, Vec3
#[derive(Debug, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}


pub fn barycentric(pts: [&Vec2<i32>; 3], p: &Vec2<i32>) -> Vec3<f64> {
    let a = Vec3::<f64>::new( (pts[2].x-pts[0].x) as f64, (pts[1].x-pts[0].x) as f64, (pts[0].x-p.x) as f64);
    let b = Vec3::<f64>::new( (pts[2].y-pts[0].y) as f64, (pts[1].y-pts[0].y) as f64, (pts[0].y-p.y) as f64);

    let u = Vec3::cross(&a, &b);

    if u.y.abs() < 1.0 { return Vec3::new(-1.0, 1.0, 1.0); }
    //return Vec3f(1.f-(u.x+u.y)/u.z, u.y/u.z, u.x/u.z); 
    let _x = 1.0 - ((u.x+u.y)/u.z);
    let _y = u.y/u.z;
    let _z = u.x/u.z;

	return Vec3 { x: _x, y: _y, z: _z };
}

// --- impl : Vec2
// new, dot, cross
impl<T> Vec2<T>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> {
        pub fn new(_x: T, _y: T) -> Vec2<T> {
            Vec2{ x: _x, y: _y }
        }

        pub fn dot(a: &Vec2<T>, b: &Vec2<T>) -> T {
            (a.x * b.x) + (a.y * b.y)
        }
}

// Vec2 operators
impl<'a, 'b, T: Copy + Add<Output = T>> Add<&'b Vec2<T>> for &'a Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, _rhs: &'b Vec2<T>) -> Vec2<T> {
        Vec2::<T> { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl<'a, 'b, T: Copy + Sub<Output = T>> Sub<&'b Vec2<T>> for &'a Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, _rhs: &'b Vec2<T>) -> Vec2<T> {
        Vec2::<T> { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}

impl<'a, 'b, T: Copy + Mul<Output = T>> Mul<T> for &'a Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, _rhs: T) -> Vec2<T> {
        Vec2::<T> { x: self.x * _rhs, y: self.y * _rhs }
    }
}

impl<'a, 'b, T: Copy + Div<Output = T>> Div<T> for &'a Vec2<T> {
    type Output = Vec2<T>;
    fn div(self, _rhs: T) -> Vec2<T> {
        Vec2::<T> { x: self.x / _rhs, y: self.y / _rhs }
    }
}

// -------------------------------------------------------------------------------------

// --- impl : Vec3
// new, dot, cross
impl<T> Vec3<T> 
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> {
        pub fn new(_x: T, _y: T, _z: T) -> Vec3<T> {
            Vec3{ x: _x, y: _y, z: _z }
        }

        pub fn dot(a: &Vec3<T>, b: &Vec3<T>) -> T {
            (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
        }

        pub fn cross(a: &Vec3<T>, b: &Vec3<T>) -> Vec3<T> {
            Vec3 { x: (a.y*b.z)-(a.z*b.y), y: (a.z*b.x)-(a.x*b.z), z: (a.x*b.y)-(a.y*b.x) }
        }
}

impl<T> Vec3<T> 
    where T: Float + Add<Output = T> + Mul<Output = T> + Div<Output = T> {
        pub fn normalize(&mut self) {
            let mag = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
            self.x = self.x / mag;
            self.y = self.y / mag;
            self.z = self.z / mag;
        }
}

// Vec3 operators
impl<'a, 'b, T: Copy + Add<Output = T>> Add<&'b Vec3<T>> for &'a Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, _rhs: &'b Vec3<T>) -> Vec3<T> {
        Vec3::<T> { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

impl<'a, 'b, T: Copy + Sub<Output = T>> Sub<&'b Vec3<T>> for &'a Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, _rhs: &'b Vec3<T>) -> Vec3<T> {
        Vec3::<T> { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl<'a, 'b, T: Copy + Mul<Output = T>> Mul<T> for &'a Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, _rhs: T) -> Vec3<T> {
        Vec3::<T> { x: self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs }
    }
}

impl<'a, 'b, T: Copy + Div<Output = T>> Div<T> for &'a Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, _rhs: T) -> Vec3<T> {
        Vec3::<T> { x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs }
    }
}

// --- tests ------------------------------------------------------------------------------
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_vec2() {
        let lhs = Vec2::new(1, 2);
        let rhs = Vec2::new(2, 3);
        let expected = Vec2::new(3, 5);
        let res_a = &lhs + &rhs;
        let res_b = &rhs + &lhs;

        assert_eq!(res_a, expected);
        assert_eq!(res_b, expected);
    }

    #[test]
    fn add_vec3() {
        let lhs = Vec3::new(1, 2, 3);
        let rhs = Vec3::new(2, 3, 1);
        let expected = Vec3::new(3, 5, 4);
        let res_a = &lhs + &rhs;
        let res_b = &rhs + &lhs;

        assert_eq!(res_a, expected);
        assert_eq!(res_b, expected);      
    }

    #[test]
    fn sub_vec2() {
        let lhs = Vec2::new(1, 2);
        let rhs = Vec2::new(2, 3);
        let expected = Vec2::new(-1, -1);
        let res_a = &lhs - &rhs;

        assert_eq!(res_a, expected);
    }

    #[test]
    fn sub_vec3() {
        let lhs = Vec3::new(1, 2, 3);
        let rhs = Vec3::new(2, 3, 1);
        let expected = Vec3::new(-1, -1, 2);
        let res_a = &lhs - &rhs;

        assert_eq!(res_a, expected);
    }

    #[test]
    fn scalar_mul_vec2() {
        let lhs = Vec2::new(1, 2);
        let rhs = 2;
        let expected = Vec2::new(2, 4);
        let res = &lhs * rhs;

        assert_eq!(res, expected);
    }

    #[test]
    fn scalar_mul_vec3() {
        let lhs = Vec3::new(1, 2, 3);
        let rhs = 2;
        let expected = Vec3::new(2, 4, 6);
        let res = &lhs * rhs;

        assert_eq!(res, expected);
    }

    #[test]
    fn scalar_div_vec2() {
        let lhs = Vec2::new(2, 4);
        let rhs = 2;
        let expected = Vec2::new(1, 2);
        let res = &lhs / rhs;

        assert_eq!(res, expected);
    }

    #[test]
    fn scalar_div_vec3() {
        let lhs = Vec3::new(2, 4, 6);
        let rhs = 2;
        let expected = Vec3::new(1, 2, 3);
        let res = &lhs / rhs;

        assert_eq!(res, expected);
    }

    #[test]
    fn dot_vec2() {
        let lhs = Vec2::new(1,2);
        let rhs = Vec2::new(2,4);
        let expected = 10;
        let res = Vec2::dot(&lhs, &rhs);

        assert_eq!(expected, res);
    }

    #[test]
    fn dot_vec3() {
        let lhs = Vec3::new(1,2,3);
        let rhs = Vec3::new(2,4,6);
        let expected = 28;
        let res = Vec3::dot(&lhs, &rhs);

        assert_eq!(expected, res);
    }

    #[test]
    fn cross_vec3() {
        let lhs = Vec3::new(-1,-2,3);
        let rhs = Vec3::new(4,0,-8);
        let expected = Vec3::new(16,4,8);
        let res = Vec3::cross(&lhs, &rhs);

        assert_eq!(expected, res);
    }

    #[test]
    fn barycentric_test() {
        let a = Vec2::new(15, 25);
        let b = Vec2::new(19, 29);
        let c = Vec2::new(16, 39);
        let pts = [&a, &b, &c];
        let p = Vec2::new(13, 11);
        let res = barycentric(pts, &p);
        let ans = Vec3::new(2.1923076923076925f64, -0.2692307692307692f64, -0.9230769230769231f64);
        assert_eq!(res, ans);

        let a = Vec2::new(401, 245);
        let b = Vec2::new(585, 277);
        let c = Vec2::new(559, 195);
        let pts = [&a, &b, &c];
        let p = Vec2::new(426, 260);
        let res = barycentric(pts, &p);
        let ans = Vec3::new(0.8835578002244668f64, 0.25392817059483724f64, -0.13748597081930417f64);
        assert_eq!(res, ans);
    }
}
use std::ops::Add;

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

// --- impl : Vec2
// new, add
impl<T> Vec2<T> {
    pub fn new(_x: T, _y: T) -> Vec2<T> {
        Vec2{ x: _x, y: _y }
    }
}

impl<'a, 'b, T: Copy + Add<Output = T>> Add<&'b Vec2<T>> for &'a Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, _rhs: &'b Vec2<T>) -> Vec2<T> {
        Vec2::<T> { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

// --- impl : Vec3
// new, add
impl<T> Vec3<T> {
    pub fn new(_x: T, _y: T, _z: T) -> Vec3<T> {
        Vec3{ x: _x, y: _y, z: _z }
    }
}

impl<'a, 'b, T: Copy + Add<Output = T>> Add<&'b Vec3<T>> for &'a Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, _rhs: &'b Vec3<T>) -> Vec3<T> {
        Vec3::<T> { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

// --- tests
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
}
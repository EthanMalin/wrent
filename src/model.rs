extern crate image;
use image::{ImageBuffer, Rgb};
use std::error;

// --- ! move file reading capabilities to utility file
use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use crate::geometry::Vec3;

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

pub struct Model {
    pub verts: Vec< Vec3<f64> >,
    pub faces: Vec< Vec3<i32> >,
}

impl Model {
    pub fn new(path: &str) -> Result<Model, Error> {
        let f = File::open(path)?;
        let buf = BufReader::new(f);

        let mut _verts: Vec< Vec3<f64> > = Vec::new();
        let mut _faces: Vec< Vec3<i32> > = Vec::new();

        for _line in buf.lines() {
            let line = match _line {
                Ok(l) => l,
                Err(e) => { return Err(e); },
            };

            let mut chars = line.chars();
            match chars.next() {
                Some('v') => {
                    match chars.next() {
                        Some(' ') => {
                            let (x, y, z) = Model::obj_parse_helper_vert(chars.as_str())?;
                            _verts.push(Vec3::new(x, y, z));
                        },
                        _ => (),
                    }

                },
                Some('f') => {
                    let triples = chars.as_str().trim().split(" ");
                    let mut f_verts = Vec::<i32>::new();
                    for triple in triples {
                        let triple_split: Vec<&str> = triple.split("/").collect();
                        f_verts.push(triple_split[0].parse::<i32>().unwrap());
                    }
                    // face indexes start at 1
                    _faces.push( Vec3::new(f_verts[0]-1, f_verts[1]-1, f_verts[2]-1) );
                },
                _ => (),
            }
        }
        
        Result::Ok( Model { verts: _verts, faces: _faces } )
    }

    fn obj_parse_helper_vert(s: &str) -> Result<(f64, f64, f64), Error> {
        let split: Vec<&str> = s.trim().split(" ").collect();
            let x = match split[0].parse::<f64>(){
                Ok(_x) => _x,
                Err(e) => { return Err(Error::new(ErrorKind::InvalidData, e.to_string())); }
            };
            let y = match split[1].parse::<f64>(){
                Ok(_y) => _y,
                Err(e) => { return Err(Error::new(ErrorKind::InvalidData, e.to_string())); }
            };
            let z = match split[2].parse::<f64>(){
                Ok(_z) => _z,
                Err(e) => { return Err(Error::new(ErrorKind::InvalidData, e.to_string())); }
            };
            return Ok((x, y, z));
    }

    pub fn n_verts(&self) -> usize {
        self.verts.len()
    }

    pub fn n_faces(&self) -> usize {
        self.verts.len()
    }

    pub fn face(&self, idx: usize) -> &Vec3<i32> {
        &self.faces[idx]
    }

    pub fn vert(&self, idx: usize) -> &Vec3<f64> {
        &self.verts[idx]
    }
}
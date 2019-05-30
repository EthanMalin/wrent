extern crate image;
use image::{ImageBuffer, Rgb};

// --- ! move file reading capabilities to utility file
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use crate::geometry::{Vec2, Vec3};

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

        for line in buf.lines() {

            let unwrapped = match line {
                Ok(l) => l,
                Err(e) => {
                    println!("error: Model::new(path = {}), errmsg: {}", path, e);
                    String::from("")
                }
            };
            if unwrapped == "" { continue; }

            // !! eventually remove these unwraps
            // !! there are potentially lots of errors here. Would be good to modularize and make safer eventually
            // !! also could be written cleaner, but that has to do with modularization
            let mut chars = unwrapped.chars();
            match chars.next().unwrap() {
                'v' => {
                    match chars.next().unwrap() {
                        ' ' => {
                            let split: Vec<&str> = chars.as_str().trim().split(" ").collect();
                            let (x,y,z) = ( split[0].parse::<f64>().unwrap(), 
                                            split[1].parse::<f64>().unwrap(), 
                                            split[2].parse::<f64>().unwrap() );
                            _verts.push(Vec3::new(x, y, z));
                        },
                        _ => (),
                    }

                },
                'f' => {
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
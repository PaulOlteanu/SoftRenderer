use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use cgmath::Vector3;

use regex::Regex;

#[derive(Debug)]
pub struct Model<'a> {
    file_path: &'a str,
    verts: Vec<Vector3<f64>>,
    faces: Vec<Vec<usize>>,
}

impl<'a> Model<'a> {
    pub fn new(file_path: &'a str) -> Self {
        let f = File::open(file_path).expect("Couldn't open obj file");
        let file = BufReader::new(&f);

        let mut verts = Vec::new();
        let mut faces = Vec::new();

        let face_regex = Regex::new(r"(\d+)(?:/\d+)+").unwrap();

        for line in file.lines() {
            let l = line.unwrap();
            let mut l = l.split_whitespace();

            match l.nth(0) {
                Some("v") => {
                    let mut vertices = Vec::new();
                    for vert in l {
                        vertices.push(vert.parse::<f64>().expect("Couldn't parse obj file"));
                    }
                    verts.push(Vector3::new(vertices[0], vertices[1], vertices[2]));
                }

                Some("f") => {
                    let mut vertices = Vec::new();
                    for vert in l {
                        let v = face_regex
                            .captures(vert)
                            .unwrap()
                            .get(1)
                            .map_or("", |m| m.as_str());

                        vertices.push(v.parse::<usize>().expect("Couldn't parse obj file") - 1);
                    }
                    faces.push(vertices);
                }

                _x => {}
            }
        }

        Self {
            file_path,
            verts,
            faces,
        }
    }

    pub fn verts(&self) -> &Vec<Vector3<f64>> {
        &self.verts
    }

    pub fn faces(&self) -> &Vec<Vec<usize>> {
        &self.faces
    }
}

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use cgmath::{Point3};

use regex::{Captures, Regex};

#[derive(Debug)]
pub struct FaceData {
    pub vertices: (usize, usize, usize),
    pub textures: (usize, usize, usize),
    pub normals: (usize, usize, usize),
}

#[derive(Debug)]
pub struct Model<'a> {
    model_file_path: &'a str,
    texture_file_path: &'a str,
    verts: Vec<Point3<f64>>,
    texture_verts: Vec<Point3<f64>>,
    normal_verts: Vec<Point3<f64>>,

    // (VertexIndex, VertexTextureIndex, VertexNormalIndex)
    faces: Vec<FaceData>,
    pub texture: image::RgbImage,
}

impl<'a> Model<'a> {
    pub fn new(model_file_path: &'a str, texture_file_path: &'a str) -> Self {
        let f = File::open(model_file_path).expect("Couldn't open obj file");
        let file = BufReader::new(&f);

        let mut verts = Vec::new();
        let mut texture_verts = Vec::new();
        let mut normal_verts = Vec::new();
        let mut faces = Vec::new();

        let face_regex = Regex::new(r"(\d+)/(\d+)/(\d+)").unwrap();

        for line in file.lines() {
            let l = line.unwrap();
            let mut l = l.split_whitespace();

            match l.nth(0) {
                Some("v") => {
                    let mut coordinates = l.map(move |x| x.parse::<f64>().unwrap());
                    verts.push(Point3::new(
                        coordinates.next().unwrap(),
                        coordinates.next().unwrap(),
                        coordinates.next().unwrap(),
                    ));
                }

                Some("vt") => {
                    let mut coordinates = l.map(move |x| x.parse::<f64>().unwrap());
                    texture_verts.push(Point3::new(
                        coordinates.next().unwrap(),
                        coordinates.next().unwrap(),
                        coordinates.next().unwrap(),
                    ));
                }

                Some("vn") => {
                    let mut coordinates = l.map(move |x| x.parse::<f64>().unwrap());
                    normal_verts.push(Point3::new(
                        coordinates.next().unwrap(),
                        coordinates.next().unwrap(),
                        coordinates.next().unwrap(),
                    ));
                }

                Some("f") => {
                    let mut face_verts = Vec::new();

                    for vert in l {
                        let vert_match =
                            face_regex.captures(vert).expect("Couldn't parse obj file");
                        face_verts.push(Self::parse_face(&vert_match));
                    }

                    let face_data = FaceData {
                        vertices: (face_verts[0].0, face_verts[1].0, face_verts[2].0),
                        textures: (face_verts[0].1, face_verts[1].1, face_verts[2].1),
                        normals: (face_verts[0].2, face_verts[1].2, face_verts[2].2),
                    };

                    faces.push(face_data);
                }

                _x => {}
            }
        }

        let texture = image::open(texture_file_path).unwrap().to_rgb();

        Self {
            model_file_path,
            texture_file_path,
            verts,
            texture_verts,
            normal_verts,
            faces,
            texture,
        }
    }

    pub fn verts(&self) -> &Vec<Point3<f64>> {
        &self.verts
    }

    pub fn texture_verts(&self) -> &Vec<Point3<f64>> {
        &self.texture_verts
    }

    pub fn normal_verts(&self) -> &Vec<Point3<f64>> {
        &self.normal_verts
    }

    pub fn faces(&self) -> &Vec<FaceData> {
        &self.faces
    }

    fn parse_face(capture: &Captures) -> (usize, usize, usize) {
        (
            capture
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .expect("Couldn't parse obj file")
                - 1,
            capture
                .get(2)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .expect("Couldn't parse obj file")
                - 1,
            capture
                .get(3)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .expect("Couldn't parse obj file")
                - 1,
        )
    }
}

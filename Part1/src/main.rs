use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

impl FromStr for Vertex {
    type Err = String;
    fn from_str(s: &str) -> Result<Vertex, String> {
        let mut words = s.split_whitespace();
        if words.next() == Some("v") {
            let floats: Result<Vec<f32>,_> = words.map(|s| s.parse()).collect();
            if let Ok(fs) = floats {
                if fs.len() == 3 { return Ok(Vertex {x: fs[0], y: fs[1], z: fs[2]}) }
            }
        }
        Err(format!("Invalid vertex: {}", s))
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v {} {} {}", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct FaceIndex {
    v1: usize,
    v2: usize,
    v3: usize,
}

impl FromStr for FaceIndex {
    type Err = String;
    fn from_str(s: &str) -> Result<FaceIndex, String> {
        let mut words = s.split_whitespace();
        if words.next() == Some("f") {
            let indices: Result<Vec<usize>,_> = words.map(|s| s.parse()).collect();
            if let Ok(is) = indices {
                if is.len() == 3 { return Ok(FaceIndex {v1: is[0], v2: is[1], v3: is[2]}) }
            }
        }
        Err(format!("Invalid face: {}", s))
    }
}

impl fmt::Display for FaceIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "f {} {} {}", self.v1, self.v2, self.v3)
    }
}

struct ObjIndex {
    vertices: Vec<Vertex>,
    faces: Vec<FaceIndex>,
}

impl FromStr for ObjIndex {
    type Err = String;
    fn from_str(data: &str) -> Result<ObjIndex,String> {
        let mut face_section = false;
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<FaceIndex> = Vec::new();

        for line in data.lines() {
            if let Some(c) = line.chars().next() {
                match c {
                    'v' => if !face_section {
                            vertices.push(try!(line.parse()))
                        } else {
                            return Err(format!("Vertex data after face: {}", line))
                        },
                    'f' => {
                        face_section = true;
                        faces.push(try!(line.parse()))
                    },
                    _ => return Err(format!("Invalid data: {}", line)),
                }
            }
        }
        Ok(ObjIndex {vertices: vertices, faces: faces})
    }
}

fn reformat_obj(obj: ObjIndex) -> String {
    let mut result = String::new();
    for v in obj.vertices {
        result.push_str(&format!("{}\n", v));
    }
    for f in obj.faces {
        result.push_str(&format!("{}\n", f));
    }
    result
}

fn main() {
    let files: Vec<File> = env::args().skip(1)
        .map(|arg| File::open(&arg).expect(&format!("Failed to open file: {}", &arg)))
        .collect();
    for (mut file, name) in files.iter().zip(env::args().skip(1)) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let obj: ObjIndex = contents.parse().unwrap();
        println!("File: {}", name);
        print!("{}", reformat_obj(obj));
    }
}

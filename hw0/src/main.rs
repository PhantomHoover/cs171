use std::env;
use std::fs::File;
use std::io::prelude::*;
mod load_obj;
use load_obj::{ObjIndex, reformat_obj};

fn main() {
    let files: Vec<File> = env::args().skip(1)
        .map(|arg| File::open(&arg).expect(&format!("Failed to open file: {}", &arg)))
        .collect();
    for (mut file, name) in files.iter().zip(env::args().skip(1)) {
        println!("File: {}", name);
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let obj: ObjIndex = contents.parse().unwrap();
        print!("{}", reformat_obj(&obj));
    }
}

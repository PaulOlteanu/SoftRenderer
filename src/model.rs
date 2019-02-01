use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub struct Model<'a> {
    file_path: &'a str,
    data: Vec<String>
}

impl<'a> Model<'a> {
    pub fn new(file_path: &'a str) -> Self {
        let mut data = Vec::new();
        let f = File::open(file_path).expect("Couldn't open obj file");
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = line.unwrap();
            data.push(l);
        }

        Self{data, file_path}
    }
}

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("info.txt").expect("Can't open file");
    let mut file2 = File::create("infoCopy.txt").expect("Expect");

    let mut cont = String::new();

    file.read_to_string(&mut cont).expect("Can not read");

    file2.write_all(cont.as_bytes()).expect("Expexted");
}

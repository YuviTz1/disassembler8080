use std::fs;
use std::path::Path;

mod utils;

fn main() {
    // reading in data from the file
    let path = Path::new("test.txt");
    let contents = fs::read(path);

    // store data in vector of bytes
    let buffer = contents.unwrap();
    let disassembly = utils::disassemble(&buffer);

    print!("{}", disassembly);
}

#[cfg(test)]
mod tests;
//#[path="tests.rs"]

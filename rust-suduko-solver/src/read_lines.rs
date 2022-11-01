use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

// Source: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

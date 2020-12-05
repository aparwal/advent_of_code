// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

// use std::{
//     fs::File,
//     io::{prelude::*, BufReader},
//     path::Path,
// };

// use anyhow::{Error, Result};
// use std::io;
// use std::str::FromStr;

// /// Reads input as lines, where every line is parsed to given type.
// pub fn input_iter<T, Input>(input: Input) -> impl Iterator<Item = Result<T>>
// where
//     T: FromStr,
//     T::Err: Into<Error>,
//     Input: io::BufRead,
// {
//     input
//         .lines()
//         .map(|item| -> Result<T> { item?.parse().map_err(Into::into) })
// }

// fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
//     let file = File::open(filename).expect("no such file");
//     let buf = BufReader::new(file);
//     buf.lines()
//         .map(|l| l.expect("Could not parse line"))
//         .collect()
// }

// use std::fs::File;
// use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

// fn read<R: Read>(input: R) -> Vec<String> {
//     let br = BufReader::new(input);
//     br.lines()
//         .map(|line| line.expect("Could not parse line"))
//         .collect()
// }

// pub fn file_to_vec_string(path :String) -> Result<Vec<String>, Error> {
//     let vec = read(File::open(path)?);
//     Ok(vec)
// }

// pub fn file_to_vec_uint(path :String) -> Result<Vec<u64>, Error> {
//     let vec = read(File::open(path)?);

//     let vec_int: Vec<u64> = vec.unwrap().into_iter().map(|string| string.parse());
//     Ok(vec)
// }

// fn read_all(file_name: &str) -> Vec<u64> {
//     std::fs::read_to_string(file_name)
//         .expect("yikes")
//         .lines()
//         .map(|x| x.parse())
//         .filter_map(std::io::Result::Ok)
//         .collect()
// }

// fn read_a_file() -> std::io::Result<Vec<u8>> {
//     let mut file = try!(File::open("example.data"));

//     let mut data = Vec::new();
//     try!(file.read_to_end(&mut data));

//     return Ok(data);
// }

// ---

use std::fs::read_to_string;
use std::str::FromStr;
pub fn file_to_vec<T: FromStr>(file_name: &str) -> Vec<T> {
  read_to_string(file_name)
    .expect("file not found!")
    .lines()
    .map(|x| x.parse())
    .flatten()
    .collect()
}

pub fn file_to_vec_split_at<T: FromStr>(file_name: &str, splitter: &str) -> Vec<T> {
  read_to_string(file_name)
    .expect("file not found")
    .split(splitter)
    .map(|x| x.parse())
    .flatten()
    .collect()
}

#[allow(dead_code)]
fn main() {
  let lines = file_to_vec::<String>("input.txt");
  for line in lines {
    println!("{:?}", line);
  }
  let lines = file_to_vec::<u64>("input.txt");
  for line in lines {
    println!("{:?}", line);
  }
}

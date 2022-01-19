use std::fs;
use std::io::prelude::*;
use std::path::Path;

mod name_parser;
use name_parser::*;

const DIR: &str = ".";
const EXT: &str = "MP4";
const PREFIX: &str = "GOPR";
const START_NUM: i32 = 1;

fn main() {
    resolve(&Path::new(DIR));

    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}

fn resolve(path: &Path) {
    if path.is_dir() {
        for subdir in fs::read_dir(path).unwrap() {
            resolve(&subdir.unwrap().path())
        }
    } else if matches(path) {
        rename(path)
    }
}

fn matches(file: &Path) -> bool {
    file.extension().map(|x| x == EXT).unwrap_or(false)
}

fn rename(path: &Path) {
    let name = path.file_name().map(|name| name.to_str().unwrap()).unwrap();

    if is_valid(name) {
        let new_name = format!(
            "{}{}_{:03}.{}",
            PREFIX,
            sequence(name),
            START_NUM + index(name),
            EXT
        );

        println!("{} -> {}", path.to_str().unwrap(), new_name);
        fs::rename(path, path.with_file_name(new_name)).unwrap();
    }
}

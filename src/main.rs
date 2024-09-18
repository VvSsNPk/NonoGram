use nono_gram::nono::NonoGram;
use nono_gram::sat_helper::AnswerParser;
use nono_gram::{create_hex, get_all_hex_directions, parse_all_files, parse_file_two, solver_nonogram};
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use varisat::Lit;

extern crate itertools;
extern crate num_traits;
pub mod nono;
pub mod sat_helper;
fn main() {
    /*let mut path = PathBuf::new();
    path.push("clues/snake-1.clues");
    let binding = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .trim_matches('"')
        .replace("clues", "solutions");
    let binding2 = path.file_name().unwrap().to_str().unwrap().trim_matches('"').replace("clues","dimacs");
    let mut path2= PathBuf::new();
    path2.push(binding2);
    /*let nono = parse_file_two(&path);
    println!("{}",nono);*/
    let result = solver_nonogram(&path,&path2);
    let ans = result.display_solution();
    let name = binding.as_str();
    let file = File::create(Path::new(name)).unwrap();
    let mut writer = BufWriter::new(file);
    for i in ans {
        writer.write(i.as_ref()).unwrap();
        writer.write("\n".as_ref()).unwrap();
    }*/
    let mut path = PathBuf::new();
    path.push("clues");
    parse_all_files(&path);
    /*let mut path1 = PathBuf::new();
    path1.push("clues");
    path1.push("ai-1.clues");
    println!("{:?}",path1);
    let mut path2 = PathBuf::new();
    path2.push("ai-1");
    path2.push("ai-1.solutions");
    println!("{:?}",path2);
    let output = Command::new("python").arg("nonogram.py").arg("visualize")
        .arg(&path1).arg(&path2).output();*/

    /*let mut path1 = PathBuf::new();
    path1.push("clues/snake-1.clues");
    let mut path2 = PathBuf::new();
    path2.push("snake-1/snake-1.solutions");
    let output = Command::new("python").arg("nonogram.py").arg("visualize")
        .arg(&path1).arg(&path2).output();
    println!("{:?}",output);*/

}

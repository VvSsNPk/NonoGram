extern crate num_traits;
pub mod nono;
pub mod sat_helper;
use crate::nono::{clue, clue::Line, NonoGram};
use crate::sat_helper::{AnswerParser, LitStore};
use nono::solver_nono::Solver;
use std::io::{BufWriter, Write};
use std::{fs, fs::File, io::{BufRead, BufReader}, path::PathBuf};
use std::fs::{create_dir, create_dir_all, read_dir, remove_dir, remove_dir_all};
use std::process::Command;

pub fn parse_file_two(path: &PathBuf) -> NonoGram {
    let file = File::open(path).expect("Not a valid path");
    let mut size = Vec::new();
    let mut clues = Vec::new();
    let mut diff = String::new();
    let mut multicolor = false;
    for (count, i) in BufReader::new(file).lines().enumerate() {
        if let Ok(string) = i {
            if count == 0 {
                let x = string.split_whitespace();
                for i in x {
                    if let Ok(num) = i.parse::<usize>() {
                        size.push(num);
                    } else {
                        i.trim().clone_into(&mut diff);
                    }
                }
            } else if count > 1 {
                let split = string.split_whitespace();
                let mut store = Vec::new();
                for i in split {
                    let mut str = i.to_string();
                    let color = str.pop();
                    let clue: usize = str.parse::<usize>().unwrap();
                    store.push((color.unwrap(), clue))
                }
                clues.push(store)
            } else {
                let split = string.split_whitespace();
                let mut store = Vec::new();
                for i in split {
                    store.push(i.to_string());
                }
                if store.len() > 2 {
                    multicolor = true;
                }
            }
        }
    }

    let first = size.first().unwrap();
    let last = size.last().unwrap();
    let mut nono_gram = NonoGram::new(*first as u32, *last as u32);
    if multicolor {
        nono_gram.set_multi_color();
    }
    if diff == "rect" {
        let mut ss = 0;
        let arr: Vec<Vec<clue::Box>> = (0..*first)
            .map(|i| {
                (0..*last)
                    .map(|j| {
                        return if multicolor {
                            ss += 2;
                            clue::Box::StoreTwo(ss - 2, ss - 1)
                        } else {
                            ss += 1;
                            clue::Box::Store(i * *last + j)
                        };
                    })
                    .collect()
            })
            .collect();
        let mut iterator = clues.iter();
        for i in arr.iter() {
            let line = Line::new(iterator.next().unwrap().clone(), i.clone());
            nono_gram.add(line);
        }
        for i in 0..*last {
            let mut clue = Vec::new();
            (0..*size.first().unwrap()).for_each(|j| {
                let num = arr[j][i].clone();
                clue.push(num)
            });
            let line = Line::new(iterator.next().unwrap().clone(), clue);
            nono_gram.add(line);
        }
    } else {
        nono_gram.set_hex();
        let hex = create_hex(*first, multicolor);
        let lines = get_all_hex_directions(&hex, *first);
        let mut iterator = clues.iter();
        for i in &hex {
            let line = Line::new(iterator.next().unwrap().clone(), i.clone());
            nono_gram.add(line);
        }
        for j in lines.iter().rev() {
            let clue = iterator.next().unwrap().clone();
            let line = Line::new(clue, j.clone());
            nono_gram.add(line);
        }
    }
    nono_gram
}

pub fn solver_nonogram(path: &PathBuf,path_to_dump: &PathBuf) -> AnswerParser{
    let nono = parse_file_two(path);
    let name = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .trim_matches('"')
        .replace("clues", "sol");
    let mut size = nono.get_size();
    let width = nono.get_width();
    let mut solver = Solver::new(nono.clone());
    solver.solve_nonogram();
    let mut sol = varisat::Solver::new();
    let cnf = solver.get_cnf();
    let file_to_write_dimacs = File::create(path_to_dump).unwrap();
    let mut buf_dimacs_writer = BufWriter::new(file_to_write_dimacs);
    varisat::dimacs::write_dimacs(&mut buf_dimacs_writer,cnf).expect("error writing dimacs file");
    sol.add_formula(&cnf);
    sol.solve().expect("error");
    let model = sol.model();
    let mut ans = model.clone().unwrap().into_iter();
    if nono.is_hex() {
        let mut answer = Vec::new();
        let sizes = nono.get_hex_sizes();
        for i in sizes {
            let mut app = Vec::new();
            for _ in 0..i {
                let apan;
                if nono.is_multi_color() {
                    let li1 = ans.next();
                    let li2 = ans.next();
                    apan = LitStore::Two(li1.unwrap(), li2.unwrap());
                } else {
                    let li = ans.next();
                    apan = LitStore::One(li.unwrap());
                }

                app.push(apan);
            }
            answer.push(app);
        }
        AnswerParser::new(nono, answer)
    } else {
        if nono.is_multi_color() {
            size = size / 2;
        }
        let mut answer = Vec::new();
        while size > 0 {
            let mut width = width;
            let mut a = Vec::new();
            while width > 0 {
                let apan;
                if nono.is_multi_color() {
                    let li1 = ans.next();
                    let li2 = ans.next();
                    apan = LitStore::Two(li1.unwrap(), li2.unwrap());
                } else {
                    let li = ans.next();
                    apan = LitStore::One(li.unwrap());
                }
                a.push(apan);
                width -= 1;
                size -= 1;
            }
            answer.push(a);
        }
        AnswerParser::new(nono, answer)
    }
}

pub fn create_hex(size: usize, multi_color: bool) -> Vec<Vec<clue::Box>> {
    let mut temp = size;
    let mut hex = Vec::new();
    let mut count = 0;
    for i in 0..size {
        let store: Vec<clue::Box> = (0..temp)
            .into_iter()
            .map(|x| {
                if multi_color {
                    count += 2;
                    clue::Box::StoreTwo(count - 2, count - 1)
                } else {
                    count += 1;
                    clue::Box::Store(count - 1)
                }
            })
            .collect();
        temp += 1;
        hex.push(store);
    }
    for j in 0..(size - 1) {
        let store: Vec<clue::Box> = (0..temp - 2)
            .into_iter()
            .map(|x| {
                if multi_color {
                    count += 2;
                    clue::Box::StoreTwo(count - 2, count - 1)
                } else {
                    count += 1;
                    clue::Box::Store(count - 1)
                }
            })
            .collect();
        temp -= 1;
        hex.push(store);
    }
    hex
}

pub fn get_all_hex_directions(hex: &Vec<Vec<clue::Box>>, size: usize) -> Vec<Vec<clue::Box>> {
    let mut iter_vec = Vec::new();
    for i in hex {
        let iter = i.into_iter();
        let iter2 = i.into_iter();
        iter_vec.push((iter, iter2));
    }
    let mut answer = Vec::new();
    let mut first = Vec::new();
    let mut second = Vec::new();
    for i in size..size * 2 {
        let mut ans = Vec::new();
        let mut ans2 = Vec::new();
        for j in 0..i {
            let mut iter = iter_vec.get_mut(j).unwrap();
            let a = iter.0.next().unwrap();
            let b = iter.1.next_back().unwrap();
            ans.push(a.clone());
            ans2.push(b.clone());
        }
        ans2.reverse();
        first.push(ans);
        second.push(ans2);
    }
    for _ in 0..size - 1 {
        let mut ans = Vec::new();
        let mut ans2 = Vec::new();
        for j in 0..size * 2 - 1 {
            let mut iter = iter_vec.get_mut(j).unwrap();
            if let Some(num) = iter.0.next() {
                ans.push(num.clone());
            }
            if let Some(num) = iter.1.next_back() {
                ans2.push(num.clone());
            }
        }
        ans2.reverse();
        first.push(ans);
        second.push(ans2);
    }
    answer.append(&mut first);
    answer.append(&mut second);
    answer
}

pub fn parse_all_files(path: &PathBuf){
    let dir = read_dir(path).expect("this is not a directory");
    let mut solutions = PathBuf::new();
    solutions.push("solutions");
    if solutions.exists(){
        remove_dir_all(&solutions).expect("no permission to remove solutions directory");
    }
    create_dir_all(&solutions).expect("couldn't create a solutions directory");
    for x in dir{
        let file = x.unwrap();
        let file_name = file.file_name();
        let file_path = file_name.to_str().unwrap().trim_matches('"').replace(".clues","");
        let solution_file = file_name.to_str().unwrap().trim_matches('"').replace("clues","solutions");
        let mut dir_path = PathBuf::new();
        dir_path.push("solutions");
        dir_path.push(file_path.clone());
        if dir_path.exists(){
            remove_dir_all(&dir_path).expect("Error directory does not exist");
        }
        create_dir(&dir_path).expect("No permissions to create solution directory");
        let dimacs_file = file_name.to_str().unwrap().trim_matches('"').replace("clues","dimacs");
        let mut dimacs_path = PathBuf::new();
        dimacs_path.push("solutions");
        dimacs_path.push(file_path);
        dimacs_path.push(dimacs_file);
        let path = file.path();
        let answer_parser = solver_nonogram(&path,&dimacs_path);
        let result = answer_parser.display_solution();


        dir_path.push(solution_file);
        let solution_file_create = File::create(&dir_path).unwrap();
        let mut writer = BufWriter::new(&solution_file_create);
        for i in result{
            writer.write(i.as_ref()).unwrap();
            writer.write("\n".as_ref()).unwrap();
        }
        writer.flush().expect("error flushing");
        let arg1 = file.path();
        let arg2 = dir_path.to_path_buf();
        let out_put = Command::new("python").arg("nonogram.py").arg("visualize").arg(&arg1).arg(&arg2).output().expect("error running command");
        let out_put2 = Command::new("python").arg("nonogram.py").arg("check").arg(&arg1).arg(&arg2).output().expect("cannot check the files");
        let string = String::from_utf8(out_put2.stdout).expect("couldnot convert the stdout to string");
        println!("{string}");
        if string.ends_with("not satisfied\r\n"){
            println!("{:?}",file_name.to_str());
        }
    }
}

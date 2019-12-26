use std::fs;
use crate::Pipe;
use crate::{X,Y};

pub fn load_pipes(file_name: &String, pipes: &mut Vec<Pipe>, field: &mut [[char;X];Y]){
    let mut i: usize = 0;
    let mut j: usize = 0;

    let file_contents = fs::read_to_string(file_name).expect("Something went wrong with file read");
    for line in file_contents.lines() {
        let s = line.split(",");
        for t in s {
                    let option = t.chars().next();
                    let mut z: char = ' ';
                    if !option.is_none() {
                        z = option.unwrap();
                        let search = pipes.iter().position(|x|x.pipe_number==z);
                        if search.is_none() {
                            pipes.push(Pipe::new(z as char,i as i32,j as i32));
                        } else {
                            pipes[search.unwrap() as usize].second_point(i as i32, j as i32);
                        }
                    }
                    field[i][j] = z;
                    i+=1;
                }
                j += 1;
                i = 0;
        }
    }




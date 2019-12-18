use std::fs;
use crate::Pipe;

pub fn load_pipes(file_name: &String, pipes: &mut Vec<Pipe>, field: &mut Vec<Vec<i16>>){
    let mut i: usize = 0;
    let mut j: usize = 0;

    let file_contents = fs::read_to_string(file_name).expect("Something went wrong with file read");
    for line in file_contents.lines() {
        let s = line.split(",");
        field.push(vec![]);
        for t in s {
            if t=="" {
                field[i].push(0);
            } else {
                let z: i16 = t.parse().unwrap();
                field[i].push(z);
                let search = pipes.iter().position(|x|x.pipe_number==z);
                if search.is_none() {
                    pipes.push(Pipe::new(z,i as i32,j as i32));
                } else {
                    pipes[search.unwrap() as usize].second_point(i as i32, j as i32);
                }
            }
            j +=1;
        }
        i +=1;
        j = 0;
    }
}



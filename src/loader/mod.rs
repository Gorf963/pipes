use std::fs;

use crate::Pipe;
use std::str::FromStr;

pub fn load_pipes(file_name: &String, pipes: &mut Vec<Pipe>, field: &mut Vec<Vec<i8>>){
    let mut i: usize = 0;
    let mut j: usize = 0;

    let file_contents = fs::read_to_string(file_name).expect("Something went wrong with file read");
    for line in file_contents.lines() {
        let s = line.split(",");

        for t in s {
            field.push(vec![]);
            if t=="" {
                field[i].push(0);
                print!(" ");
            } else {
                print!("{}",t);
                let z: i8 = i8::from_str(t).unwrap();
                field[i].push(z)
            }
            j +=1;
        }
        println!("");
        i +=1;
        j = 0;
    }

    println!("{}",j);
    let mut p = Pipe::new(0,1,1);
    p.second_point(6, 6);
    pipes.push(p);
    let mut p = Pipe::new(1,2,1);
    p.second_point(3,2);
    pipes.push(p);
    let mut p = Pipe::new(3,3,1);
    p.second_point(5,5);
    pipes.push(p);

}

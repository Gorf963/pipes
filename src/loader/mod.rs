use std::fs;

use crate::Pipe;

pub fn load_pipes(file_name: &String, pipes: &mut Vec<Pipe>, field: &mut Vec<Vec<i8>>){

    let file_contents = fs::read_to_string(file_name).expect("Something went wrong with file read");
    for line in file_contents.lines() {
        let s = line.split(",");
        field[0].push(1);        

        for t in s {
            
            if t=="" {
            
                print!(" ");
            } else {
                print!("{}",t);
            }
        }
        println!("");
    }

    
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

pub(crate) const X: usize = 95;
pub(crate) const Y: usize = 95;


mod pipe;
mod loader;

use pipe::Pipe;
use loader::load_pipes;
use std::env;
use std::time::Instant;

fn main() {

    let mut field = [['_'; X];Y];
    
    let mut pipes: Vec<Pipe> = Vec::new();

    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let start = Instant::now();
    load_pipes(file_name, &mut pipes, &mut field);
    let load_duration = start.elapsed();
    let start = Instant::now();
    pipes.sort_by(|a,b| b.total_len.cmp(&a.total_len).reverse());
    for mut pipe in pipes {
        println!("{}",pipe.to_string());
        if !pipe.find_path(&mut field)
        {
            println!("Could not solve pipe number {}", pipe.pipe_number);
            return;
        } 
        }
    let process_duration = start.elapsed();
    for j in {0..Y} {
        for i in {0..X} {

            print!("{}",field[i][j]);
        }
        println!("");
    }
    println!("Load took {:?}",load_duration);
    println!("Process took {:?}", process_duration)
}


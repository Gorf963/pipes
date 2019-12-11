mod pipe;
mod loader;
//use loader::loader;

use pipe::Pipe;
use loader::load_pipes;
use std::env;

fn main() {
    let mut field = vec![vec![0]];
    let mut pipes: Vec<Pipe> = Vec::new();

    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];


    load_pipes(file_name, &mut pipes, &mut field);
    pipes.sort_by(|a,b| b.total_len.cmp(&a.total_len));

    for pipe in pipes {
        println!("{}",pipe.to_string());
    }

}

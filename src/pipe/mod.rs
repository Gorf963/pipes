//pipe
use std::{i32};
use std::cmp::min;
use crate::{X,Y};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct StepPipe {
    x: i32,
    y: i32,
}
impl StepPipe {
    fn new(ax:i32, ay:i32) ->StepPipe {
        StepPipe {
        x:ax,
        y:ay,
        }
    }

    fn is_empty(&mut self) -> bool {
        if self.x==-1&&self.y==-1 {return true;}
        return false;

    }

    fn len(&mut self, x:i32, y:i32) -> i32 {
        if self.x == -1 || self.y ==-1 {return (10*X)as i32;}
        return (self.x - x).abs() + (self.y - y).abs();
    }
    fn clone(&mut self) -> StepPipe {
        StepPipe {
            x:self.x,
            y:self.y,
        }
    }
}
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pipe {
    pub pipe_number: char,
    start_x:i32,
    start_y:i32,
    end_x:i32,
    end_y:i32,
    pub len_x:i32,
    pub len_y:i32,
    pub total_len:i32,
}
impl Pipe {
    pub fn new(pipe_number:char, start_x:i32, start_y:i32) ->Pipe {
        Pipe {
            pipe_number: pipe_number, 
            start_x: start_x,
            start_y: start_y,
            end_x:-1,
            end_y:-1,
            len_x:0,
            len_y:0,
            total_len:0,
        }
    }
    pub fn second_point(&mut self, end_x:i32, end_y:i32) -> &Pipe {
        self.end_x=end_x;
        self.end_y=end_y;
        self.len_x = i32::abs(self.end_x - self.start_x);
        self.len_y = i32::abs(self.end_y - self.start_y);
        self.total_len = self.len_x+self.len_y;
        return self;
    }
    pub fn to_string(&self) -> String {
        let mut _output = String::new();
        _output = format!("Pipe: {}, X0:{} Y0:{},X1:{}, Y1:{}", self.pipe_number, self.start_x, self.start_y, self.end_x, self.end_y);
        return _output;
    }
    pub fn find_path(&mut self, field: &mut [[char;X];Y]) -> bool
    {
        let mut solved = false;
        let mut path = vec!(StepPipe::new(self.start_x, self.start_y));

        let mut north:StepPipe;
        let mut south:StepPipe;
        let mut east:StepPipe;
        let mut west:StepPipe;
        let mut current_step:StepPipe;

        current_step = StepPipe::new(self.start_x, self.start_y);

        if self.end_x==-1 || self.end_y ==-1 {
            return false;
        }

        while !solved
         {
            //determine potential next steps

            //north
            if current_step.y>0 {
                let a = field[current_step.x as usize][(current_step.y -1) as usize];
                if a==self.pipe_number && current_step.x==self.end_x && current_step.y-1==self.end_y {
                    north = StepPipe::new(current_step.x,current_step.y-1);
                } else {                
                    if a==' ' || a!=self.pipe_number {
                        north = StepPipe::new(current_step.x,current_step.y-1);
                    } else {
                        north = StepPipe::new(-1,-1);
                    }
                }
            } else {
                north = StepPipe::new(-1,-1);
            }
        
            //south
            if current_step.y < (Y-1) as i32 {
                let a = field[current_step.x as usize][(current_step.y +1) as usize];
                if a==self.pipe_number && current_step.x==self.end_x && current_step.y+1==self.end_y {
                    south = StepPipe::new(current_step.x,current_step.y+1);
                } else {                
                    if a==' ' || a!=self.pipe_number {
                        south = StepPipe::new(current_step.x,current_step.y+1);
                    } else {
                        south = StepPipe::new(-1,-1);
                    }
                }

            } else {
                south = StepPipe::new(-1,-1);
            }
            //east
            if current_step.x > 0 as i32 {
                let a = field[(current_step.x -1) as usize][current_step.y as usize];
                if a==self.pipe_number && current_step.x-1==self.end_x && current_step.y==self.end_y {
                    east = StepPipe::new(current_step.x-1,current_step.y);
                } else {
                    if a==' ' || a!=self.pipe_number {
                        east = StepPipe::new(current_step.x-1,current_step.y);
                    } else {
                        east = StepPipe::new(-1,-1);
                    }
                }

            } else {
                east = StepPipe::new(-1,-1);
            }

            //west
            if current_step.x < (X-1) as i32 {
                let a = field[(current_step.x +1) as usize][current_step.y as usize];
                if a==self.pipe_number && current_step.x+1==self.end_x && current_step.y==self.end_y {
                    west = StepPipe::new(current_step.x+1,current_step.y);
                } else {
                    if a==' ' || a!=self.pipe_number {
                        west = StepPipe::new(current_step.x+1,current_step.y);
                    } else {
                        west = StepPipe::new(-1,-1);
                    }
                }

            } else {
                west = StepPipe::new(-1,-1);
            }

            //pick next step

            path.push(picknext(&mut north, &mut south, &mut east, &mut west, self.end_x, self.end_y));
            
            //set step
            current_step = path.last().cloned().unwrap();
            if current_step.is_empty() {return false;}

            if current_step.x == self.end_x && current_step.y == self.end_y {
                solved = true;
            }
            field[current_step.x as usize][current_step.y as usize] = self.pipe_number;

        }
        return true;
    }
}

fn picknext(north:&mut StepPipe, south:&mut StepPipe, east:&mut StepPipe, west:&mut StepPipe, x:i32, y:i32) -> StepPipe{
    let ln = north.len(x, y);
    let ls = south.len(x, y);
    let le = east.len(x, y);
    let lw = west.len(x, y);

    let mut l = min(ln,ls);
    l = min(l,le);
    l = min(l,lw);
    if l == (10*y) as i32 {return StepPipe::new(-1,-1);}

    if l==ln {return north.clone();}
    if l==ls {return south.clone();}
    if l==le {return east.clone();}
    if l==lw {return west.clone();}

    return north.clone();
    
}
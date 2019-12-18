//pipe
use std::i32;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pipe {
    pub pipe_number: i16,
    start_x:i32,
    start_y:i32,
    end_x:i32,
    end_y:i32,
    pub len_x:i32,
    pub len_y:i32,
    pub total_len:i32,
    current_x: i32,
    current_y: i32,
}
impl Pipe {
    pub fn new(pipe_number:i16, start_x:i32, start_y:i32) ->Pipe {
        Pipe {
            pipe_number: pipe_number, 
            start_x: start_x,
            start_y: start_y,
            end_x:-1,
            end_y:-1,
            len_x:0,
            len_y:0,
            total_len:0,
            current_x:0,
            current_y:0,
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
    pub fn find_path(&mut self, field: &mut Vec<Vec<i16>>) -> bool
    {
        let mut solved = false;
        if self.end_x==-1 || self.end_y ==-1 {
            return false;
        }
        self.current_x = self.start_x;
        self.current_y = self.start_y;

        while !solved
         {
            if self.len_x > self.len_y {
                if self.len_x > 0
                {
                    self.current_x += 1;
                    
                } else
                {
                    self.current_x += -1;
                }
                self.len_x += -1;
            } else {
                if self.len_y > 0 {
                    self.current_y += 1
                } else {
                    self.current_y += -1
                }
                self.len_y += -1;
            }
            field[self.current_x as usize][self.current_y as usize] = self.pipe_number;
            if self.current_x == self.end_x && self.current_y == self.end_y {
                solved = true;
            }
        }
        return true;
    }
}

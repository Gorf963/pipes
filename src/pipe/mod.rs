//pipe

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pipe {
    pipe_number: i16,
    start_x:i32,
    start_y:i32,
    end_x:i32,
    end_y:i32,
    pub len_x:i32,
    pub len_y:i32,
    pub total_len:i32,
}
impl Pipe {
    pub fn new(pipe_number:i16, start_x:i32, start_y:i32) ->Pipe {
        Pipe {
            pipe_number: pipe_number, 
            start_x: start_x,
            start_y: start_y,
            end_x:0,
            end_y:0,
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
}

pub struct Square{
    pub width: u32,
}

impl Square{
    pub fn new(size:u32)->Self{
        Self{
            width:size
        }
    }
    pub fn area(&self)->u32{
        self.width * self.width
    }
}
pub struct TestStruct{
    pub count : i32
}

impl TestStruct {
    pub fn up(&mut self){
        self.count = self.count + 1;
    }
}
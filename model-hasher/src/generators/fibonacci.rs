pub struct Fibonacci {
    current: u128,
    next: u128 ,

    offset: u32,
    // Lucas Numbers (custom base)
    //base_current: u128,
    //base_next: u128
}
impl Fibonacci {
    pub fn new(offset: u32)->Self{
        Fibonacci{
            current: 0,
            next: 1,
            offset: offset
        }
    }
    pub fn reset(&mut self){
        self.current = 0;
        self.next = 1;
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci::new(0)
    }
}
impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item>{
        let new_next = self.current.saturating_add(self.next);
        self.current = self.next;
        self.next = new_next;

        Some(self.current + self.offset as u128)
    }
}

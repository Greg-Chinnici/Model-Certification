pub struct Fibonacci {
    current: u64,
    next: u64 ,

    offset: u32,
    when_to_wrap: u64
    // Lucas Numbers (custom base)
    //base_current: u128,
    //base_next: u128
}
impl Fibonacci {
    pub fn new(offset: u32)->Self{
        Fibonacci{
            current: 0,
            next: 1,
            offset: offset,
            when_to_wrap: 2^20 // max possible value
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
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item>{
        let mut new_next = self.current.saturating_add(self.next);
        if new_next >= self.when_to_wrap {self.reset(); new_next=self.current+self.next}

        self.current = self.next;
        self.next = new_next;

        Some(self.current + self.offset as u64)
    }
}

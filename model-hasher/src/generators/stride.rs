pub struct Stride{
    current: u64,
    gap: usize
}

impl Stride {
    pub fn new(start: u64 , gap: usize)->Self{
        Stride{
            current: start,
            gap: gap,
        }
    }
    pub fn reset(&mut self){
        self.current = 0;
    }
}

impl Iterator for Stride {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.current = self.current.saturating_add(self.gap as u64);
        Some(self.current)
    }
}

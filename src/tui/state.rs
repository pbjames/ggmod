use std::iter::Cycle;

pub struct CyclicState<I, T> {
    cycle: Cycle<I>,
    len: usize,
    pub item: T,
}

impl<I, T> CyclicState<I, T>
where
    I: Clone + Iterator<Item = T>,
{
    pub fn new(iter: I, item: T) -> Self {
        Self {
            cycle: iter.clone().cycle(),
            item,
            len: iter.count(),
        }
    }

    pub fn cycle(&mut self) {
        self.item = self.cycle.next().unwrap()
    }

    pub fn cycle_back(&mut self) {
        for _ in 0..self.len - 2 {
            self.cycle.next();
        }
        self.cycle();
    }
}

#[derive(Clone)]
pub struct Grid {
    w: usize,
    h: usize,
    cells: Vec<u8>,
}

impl Grid {
    pub fn new(w: usize, h: usize) -> Self {
        Self { w, h, cells: vec![0; w * h] }
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }


    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.w && y < self.h);
        y * self.w + x
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        let i = self.idx(x, y);
        self.cells[i]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        let i = self.idx(x, y);
        self.cells[i] = value;
    }

    pub fn fill(&mut self, value: u8) {
        self.cells.fill(value);
    }

    pub fn swap(&mut self, other: &mut Grid) {
        std::mem::swap(&mut self.cells, &mut other.cells);
    }
}

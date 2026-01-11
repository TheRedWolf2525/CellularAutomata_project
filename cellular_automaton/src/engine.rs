use crate::{automaton::Automaton, grid::Grid};

pub struct Engine {
    current: Grid,
    next: Grid,
    automaton: Box<dyn Automaton>,
}

impl Engine {
    pub fn new(w: usize, h: usize, automaton: Box<dyn Automaton>) -> Self {
        let mut current = Grid::new(w, h);
        let next = Grid::new(w, h);
        automaton.init(&mut current);
        Self { current, next, automaton }
    }

    pub fn current(&self) -> &Grid {
        &self.current
    }

    pub fn set_automaton(&mut self, automaton: Box<dyn Automaton>) {
        self.automaton = automaton;
        self.automaton.init(&mut self.current);
        self.next = Grid::new(self.current.width(), self.current.height());
    }

    pub fn step_once(&mut self) {
        self.automaton.step(&self.current, &mut self.next);
        self.current.swap(&mut self.next);
    }

    pub fn set_grid(&mut self, grid: Grid) {
        self.current = grid;
        self.next = Grid::new(self.current.width(), self.current.height());
    }
}

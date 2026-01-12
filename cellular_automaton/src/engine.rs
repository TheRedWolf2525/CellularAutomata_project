use crate::{automaton::Automaton, grid::Grid};

pub struct Engine {
    current: Grid,
    next: Grid,
    automaton: Box<dyn Automaton>,
    async_fact: f32,
}

impl Engine {
    pub fn new(w: usize, h: usize, async_fact: f32, automaton: Box<dyn Automaton>) -> Self {
        let mut current = Grid::new(w, h);
        let next = Grid::new(w, h);
        automaton.init(&mut current);
        Self { current, next, automaton, async_fact }
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
        self.automaton.step(&self.current, &mut self.next, self.async_fact);
        self.current.swap(&mut self.next);
    }

    pub fn soft_init(&mut self) {
        self.automaton.soft_init(&mut self.current);
    }

    pub fn set_grid(&mut self, grid: Grid) {
        self.current = grid;
        self.next = Grid::new(self.current.width(), self.current.height());
    }

    pub fn set_async_fact(&mut self, async_fact: f32) {
        self.async_fact = async_fact;
    }
}

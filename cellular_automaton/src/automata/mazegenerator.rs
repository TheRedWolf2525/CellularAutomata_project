use crate::{automaton::Automaton, grid::Grid};
use rand::{Rng, rng};
use std::cell::{Cell, RefCell};

pub struct MazeGenerator {
    started: Cell<bool>,
    stack: RefCell<Vec<(usize, usize)>>,
}

impl MazeGenerator {
    pub fn new() -> Self {
        Self {
            started: Cell::new(false),
            stack: RefCell::new(Vec::new()),
        }
    }
}


impl Automaton for MazeGenerator {
    fn name(&self) -> &'static str {"generator"}

    fn init(&self, grid: &mut Grid){
        grid.fill(1);
    }

    fn soft_init(&self, _grid: &mut Grid) {}

    fn step(&self, current: &Grid, next: &mut Grid, _async_fact: f32){
        for y in 0..current.height() {
            for x in 0..current.width() {
                next.set(x, y, current.get(x, y));
            }
        }

        // --- init DFS (une seule fois)
        if !self.started.get() {
            self.started.set(true);

            let sx = 1;
            let sy = 1;

            next.set(sx, sy, 0);
            self.stack.borrow_mut().push((sx, sy));
            return;
        }

        // --- DFS : un pas par step
        let mut rng = rng();
        let mut stack = self.stack.borrow_mut();

        if let Some(&(x, y)) = stack.last() {
            let w = next.width();
            let h = next.height();

            // voisins à distance 2 (cellules impaires)
            let mut neigh = Vec::new();
            let dirs = [(-2i32, 0i32), (2, 0), (0, -2), (0, 2)];

            for (dx, dy) in dirs {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx > 0 && ny > 0 && (nx as usize) < w - 1 && (ny as usize) < h - 1 {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if next.get(nx, ny) == 1 {
                        neigh.push((nx, ny));
                    }
                }
            }

            if neigh.is_empty() {
                // backtrack
                stack.pop();
            } else {
                // creuse vers un voisin aléatoire
                let (nx, ny) = neigh[rng.random_range(0..neigh.len())];
                let wx = (x + nx) / 2;
                let wy = (y + ny) / 2;

                next.set(wx, wy, 0); // casse le mur
                next.set(nx, ny, 0); // nouvelle cellule
                stack.push((nx, ny));
            }
        }
    }
}
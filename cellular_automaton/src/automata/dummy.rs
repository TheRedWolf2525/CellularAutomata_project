use crate::{automaton::Automaton, grid::Grid};

pub struct Dummy;

impl Automaton for Dummy {
    fn name(&self) -> &'static str { "dummy" }

    fn init(&self, grid: &mut Grid) {
        // petit motif: un rectangle plein
        let w = grid.width();
        let h = grid.height();
        for y in h / 3..(2 * h / 3) {
            for x in w / 3..(2 * w / 3) {
                grid.set(x, y, 1);
            }
        }
    }

    fn soft_init(&self, _grid: &mut Grid) {}

    fn step(&self, current: &Grid, next: &mut Grid, async_fact: f32) {
        // exemple simple: inversion 0<->1
        for y in 0..current.height() {
            for x in 0..current.width() {
                let v = current.get(x, y);
                next.set(x, y, if v == 0 { 1 } else { 0 });
            }
        }
    }
}

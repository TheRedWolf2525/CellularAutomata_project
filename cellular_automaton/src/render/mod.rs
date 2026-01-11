pub mod terminal;

use crate::grid::Grid;

pub trait Renderer {
    fn name(&self) -> &'static str;
    fn render(&mut self, grid: &Grid);
}

use crate::grid::Grid;

pub trait Automaton {
    fn name(&self) -> &'static str;

    /// Initialise la grille (seed).
    fn init(&self, grid: &mut Grid);

    /// Calcule l'état suivant dans `next` à partir de `current`.
    fn step(&self, current: &Grid, next: &mut Grid);
}

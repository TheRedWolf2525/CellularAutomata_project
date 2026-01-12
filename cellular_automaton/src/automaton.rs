use crate::grid::Grid;

pub trait Automaton {
    fn name(&self) -> &'static str;

    // Initialise la grille (seed).
    fn init(&self, grid: &mut Grid);

    // Initialisation au chargement d'une grille
    fn soft_init(&self, grid: &mut Grid);

    // Calcule l'état suivant dans `next` à partir de `current`.
    fn step(&self, current: &Grid, next: &mut Grid, async_fact: f32);
}

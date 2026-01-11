use std::io::{self, Write};

use crate::{grid::Grid, render::Renderer};

pub struct TerminalRenderer;

impl TerminalRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for TerminalRenderer {
    fn name(&self) -> &'static str { "terminal" }

    fn render(&mut self, grid: &Grid) {
        // efface écran + curseur en haut
        print!("\x1b[2J\x1b[H");

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let v = grid.get(x, y);
                let ch = if v == 0 { ' ' } else { '█' };
                print!("{ch}");
            }
            println!();
        }

        io::stdout().flush().ok();
    }
}

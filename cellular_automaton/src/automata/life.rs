use crate::{automaton::Automaton, grid::Grid};

pub struct Life;

impl Life {
    #[inline]
    fn count_neighbors(cur: &Grid, x: usize, y: usize) -> u8 {
        let w = cur.width();
        let h = cur.height();

        // tore (wrap)
        let xm1 = if x == 0 { w - 1 } else { x - 1 };
        let xp1 = if x + 1 == w { 0 } else { x + 1 };
        let ym1 = if y == 0 { h - 1 } else { y - 1 };
        let yp1 = if y + 1 == h { 0 } else { y + 1 };

        let mut n = 0u8;
        n += (cur.get(xm1, ym1) != 0) as u8;
        n += (cur.get(x,   ym1) != 0) as u8;
        n += (cur.get(xp1, ym1) != 0) as u8;

        n += (cur.get(xm1, y) != 0) as u8;
        n += (cur.get(xp1, y) != 0) as u8;

        n += (cur.get(xm1, yp1) != 0) as u8;
        n += (cur.get(x,   yp1) != 0) as u8;
        n += (cur.get(xp1, yp1) != 0) as u8;
        
        n
    }
}

impl Automaton for Life {
    fn name(&self) -> &'static str { "life" }

    fn init(&self, grid: &mut Grid) {
        grid.fill(0);

        // un "glider" simple pour tester
        let (x, y) = (2, 2);
        grid.set(x + 1, y, 1);
        grid.set(x + 2, y + 1, 1);
        grid.set(x,     y + 2, 1);
        grid.set(x + 1, y + 2, 1);
        grid.set(x + 2, y + 2, 1);
    }

    fn step(&self, current: &Grid, next: &mut Grid) {
        for y in 0..current.height() {
            for x in 0..current.width() {
                let alive = current.get(x, y) != 0;
                let n = Self::count_neighbors(current, x, y);

                let out_alive = match (alive, n) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                next.set(x, y, if out_alive { 1 } else { 0 });
            }
        }
    }
}

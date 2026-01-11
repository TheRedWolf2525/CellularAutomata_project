use crate::{automaton::Automaton, grid::Grid};

pub struct MazeSolver;

/*
0 -> chemin
1 -> murs
2 -> départ
3 -> arrivée
4 -> exploration
5 -> parcouru
6 -> suppression
7 -> trajet
*/

impl MazeSolver {
    #[inline]
    fn get_neighbors(cur: &Grid, x: usize, y: usize) -> Vec<u8> {
        let w = cur.width();
        let h = cur.height();

        // tore (wrap)
        let xm1 = if x == 0 { w - 1 } else { x - 1 };
        let xp1 = if x + 1 == w { 0 } else { x + 1 };
        let ym1 = if y == 0 { h - 1 } else { y - 1 };
        let yp1 = if y + 1 == h { 0 } else { y + 1 };

        let mut l = vec![0;4];
        l[0] = (cur.get(x,   ym1) != 0) as u8;
        l[1] = (cur.get(xm1, y) != 0) as u8;
        l[2] = (cur.get(xp1, y) != 0) as u8;
        l[3] = (cur.get(x,   yp1) != 0) as u8;
                
        l
    }

    #[inline]
    fn in_neighbors(cur: &Grid, x: usize, y: usize, celltype: u8) -> bool {
        let w = cur.width();
        let h = cur.height();

        // tore (wrap)
        let xm1 = if x == 0 { w - 1 } else { x - 1 };
        let xp1 = if x + 1 == w { 0 } else { x + 1 };
        let ym1 = if y == 0 { h - 1 } else { y - 1 };
        let yp1 = if y + 1 == h { 0 } else { y + 1 };

        if cur.get(x, ym1) == celltype {return true;}
        if cur.get(xm1, y) == celltype {return true;}
        if cur.get(xp1, y) == celltype {return true;}
        if cur.get(x, yp1) == celltype {return true;}
                
        false
    }
}

impl Automaton for MazeSolver{
    fn name(&self) -> &'static str { "mazesolver" }

    fn init(&self, grid: &mut Grid) {
        let w = grid.width();
        let h = grid.height();

        grid.fill(0);

        for x in 0..w {
            grid.set(x, 0, 1);
            grid.set(x, h - 1, 1);
        }

        for y in 0..h {
            grid.set(0, y, 1);
            grid.set(w - 1, y, 1);
        }

        grid.set(1, 1, 2);
        grid.set(w-10, h-10, 3);
    }


    fn step(&self, current: &Grid, next: &mut Grid) {        
        for y in 0..current.height() {
            for x in 0..current.width() {
                let v = current.get(x, y);
                next.set(x, y, v); // par défaut même valeur

                // Exploration
                if v == 0 && (Self::in_neighbors(current, x, y, 2) || Self::in_neighbors(current, x, y, 4)) {next.set(x, y, 4);}
                if v == 4 && !Self::in_neighbors(current, x, y, 0) {next.set(x, y, 5);}

                if v == 4 && Self::in_neighbors(current, x, y, 3) {next.set(x, y, 7);}

                // Backtracking
                if v == 5 && Self::in_neighbors(current, x, y, 7) {next.set(x, y, 7);}
                if v == 5 && !Self::in_neighbors(current, x, y, 7) && (Self::in_neighbors(current, x, y, 6) || Self::in_neighbors(current, x, y, 0)) {next.set(x, y, 7);}

                // Suppression
                if (v == 4 || v == 5) && Self::in_neighbors(current, x, y, 6) {next.set(x, y, 6);}
                if v == 6 {next.set(x, y, 0);}
            }
        }
    }
}
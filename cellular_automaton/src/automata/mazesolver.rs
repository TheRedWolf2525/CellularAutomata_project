use crate::{automaton::Automaton, grid::Grid, automata::patterns::ALL_PATTERNS};
use rand::{Rng, rng};

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
    fn get_4neigh_count(cur: &Grid, x: usize, y: usize) -> [u8; 8] {
        let w = cur.width();
        let h = cur.height();

        // tore (wrap)
        let xm1 = if x == 0 { w - 1 } else { x - 1 };
        let xp1 = if x + 1 == w { 0 } else { x + 1 };
        let ym1 = if y == 0 { h - 1 } else { y - 1 };
        let yp1 = if y + 1 == h { 0 } else { y + 1 };

        let mut l = [0u8; 8];
        let n = cur.get(x,   ym1) as usize;
        l[n] += 1;
        let n = cur.get(xm1, y) as usize;
        l[n] += 1;
        let n = cur.get(xp1, y) as usize;
        l[n] += 1;
        let n = cur.get(x,   yp1) as usize;
        l[n] += 1;
        
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

    #[inline]
    fn match_pattern(cur: &Grid, x: usize, y: usize, pat: &[[i8; 5]; 5]) -> bool {
        let w = cur.width() as i32;
        let h = cur.height() as i32;
        let cx = x as i32;
        let cy = y as i32;

        #[inline]
        fn wrap(a: i32, m: i32) -> usize {
            a.rem_euclid(m) as usize
        }

        // 8 symétries du carré (D4) sur les indices (i,j) du pattern 5x5
        // On parcourt la fenêtre 5x5 dans la grille, et on compare à pat[pj][pi].
        for t in 0..8u8 {
            let mut ok = true;

            for j in 0..5usize {
                for i in 0..5usize {
                    // Coord grille de la case (i,j) de la fenêtre (wrap)
                    let gx = wrap(cx + (i as i32 - 2), w);
                    let gy = wrap(cy + (j as i32 - 2), h);

                    // Indices dans le pattern après transformation
                    let (pi, pj) = match t {
                        0 => (i, j),               // identité
                        1 => (4 - j, i),           // rot 90
                        2 => (4 - i, 4 - j),       // rot 180
                        3 => (j, 4 - i),           // rot 270
                        4 => (4 - i, j),           // miroir vertical
                        5 => (4 - j, 4 - i),       // miroir vertical + rot 90
                        6 => (i, 4 - j),           // miroir vertical + rot 180 (miroir horizontal)
                        _ => (j, i),               // miroir vertical + rot 270 (miroir diag)
                    };

                    let want = pat[pj][pi];
                    if want == -1 {
                        continue; // wildcard
                    }

                    if cur.get(gx, gy) != want as u8 {
                        ok = false;
                        break;
                    }
                }
                if !ok { break; }
            }

            if ok {
                return true;
            }
        }

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
            if x < 60 {grid.set(x, 15, 1);}
            if x > 30 {grid.set(x, 25, 1);}
        }

        for y in 0..h {
            grid.set(0, y, 1);
            grid.set(w - 1, y, 1);
        }

        self.soft_init(grid);
    }

    fn soft_init(&self, grid: &mut Grid) {
        let w = grid.width();
        let h = grid.height();

        grid.set(1, 1, 2);
        grid.set(w-9, h-10, 3);
    }

    fn step(&self, current: &Grid, next: &mut Grid, async_fact: f32) {        
        if async_fact == 0.0 {
            let x = rng().random_range(..current.width());
            let y = rng().random_range(..current.height());

            let v = current.get(x, y);
            next.set(x, y, v); // par défaut même valeur

            // Exploration
            if v == 0 && (Self::in_neighbors(current, x, y, 2) || Self::in_neighbors(current, x, y, 4)) {next.set(x, y, 4);}
            if v == 4 {next.set(x, y, 5);}

            if v == 4 && Self::in_neighbors(current, x, y, 3) {next.set(x, y, 7);}

            // Backtracking
            if v == 5 && Self::in_neighbors(current, x, y, 7) {next.set(x, y, 7);}
            
            // Suppression
            let l = Self::get_4neigh_count(current, x, y);
            if v == 7 {
                if (l[1]+l[0])>=3 {next.set(x, y, 6);}
                if (l[1]+l[0])>=2 && l[6]>=1 {next.set(x, y, 6);}
                for pat in ALL_PATTERNS {
                    if Self::match_pattern(current, x, y, pat){
                        next.set(x, y, 6);
                        break;
                    }
                }
            }
            if v == 6 {next.set(x, y, 0);}
        } else {
            for y in 0..current.height() {
                for x in 0..current.width() {
                    if rng().random::<f32>() < async_fact {
                        let v = current.get(x, y);
                        next.set(x, y, v); // par défaut même valeur
        
                        // Exploration
                        if v == 0 && (Self::in_neighbors(current, x, y, 2) || Self::in_neighbors(current, x, y, 4)) {next.set(x, y, 4);}
                        if v == 4 {next.set(x, y, 5);}
        
                        if v == 4 && Self::in_neighbors(current, x, y, 3) {next.set(x, y, 7);}
        
                        // Backtracking
                        if v == 5 && Self::in_neighbors(current, x, y, 7) {next.set(x, y, 7);}
                        
                        // Suppression
                        let l = Self::get_4neigh_count(current, x, y);
                        if v == 7 {
                            if (l[1]+l[0])>=3 {next.set(x, y, 6);}
                            if (l[1]+l[0])>=2 && l[6]>=1 {next.set(x, y, 6);}
                            for pat in ALL_PATTERNS {
                                if Self::match_pattern(current, x, y, pat){
                                    next.set(x, y, 6);
                                    break;
                                }
                            }
                        }
                        if v == 6 {next.set(x, y, 0);}
                    } else {
                        next.set(x, y, current.get(x, y));
                    }
                }
            }
        }
    }
}
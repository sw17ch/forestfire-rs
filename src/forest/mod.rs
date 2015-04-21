use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Forest {
    pub trees: Vec<TreeState>,
    width: usize,
    height: usize,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TreeState {
    Alive,
    Burning,
    Burned,
}

// A coordinate in a grid.
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

macro_rules! coord {
    ($x:expr, $y:expr) => (Coord { x:$x, y:$y });
}

impl Forest {
    pub fn new(width: usize, height: usize) -> Forest {
        let v = vec![TreeState::Alive; (width * height)];

        Forest { width: width, height: height, trees: v }
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn light(&mut self) {
        let c = coord!(self.width / 2, self.height / 2);
        let center = coord_ix(self.width, c);
        self.trees[center] = TreeState::Burning;
    }

    pub fn tree_at(&self, c: Coord) -> &TreeState {
        &self.trees[coord_ix(self.width, c)]
    }

    pub fn burn(&mut self, prob_spread: f64, prob_burn_out: f64) {
        let mut v: Vec<TreeState> = Vec::with_capacity(self.trees.len());

        let mut ix: usize = 0;
        for t in &self.trees[..] {
            let new_state = match t {
                &TreeState::Alive => {
                    let bns = self.num_burning_neighbors(ix_coord(self.width, ix));
                    if ::lighter::spark(bns, prob_spread) {
                        TreeState::Burning
                    } else {
                        TreeState::Alive
                    }
                },
                &TreeState::Burning => if ::lighter::burn_out(prob_burn_out) {
                    TreeState::Burned
                } else {
                    TreeState::Burning
                },
                &TreeState::Burned => TreeState::Burned,
            };

            v.push(new_state);
            ix += 1;
        }
        self.trees = v;
    }

    pub fn burning(&self) -> bool {
        for t in &self.trees[..] {
            if t == &TreeState::Burning {
                return true;
            }
        }

        return false;
    }

    fn num_burning_neighbors(&self, c: Coord) -> usize {
        let mut burning = 0;

        for s in self.neighbor_states(c) {
            if TreeState::Burning == s {
                burning += 1;
            }
        }

        burning
    }

    fn neighbor_states(&self, c: Coord) -> Vec<TreeState> {
        let ns = neighbors(self.width, self.height, c);
        let ts = &self.trees[..];
        ns.iter().map(|n| { ts[*n] }).collect()
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "FOREST {} {}", self.width, self.height).ok();
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.trees[coord_ix(self.width, coord!(x,y))]).ok();
            }
            writeln!(f, "").ok();
        }
        Ok(())
    }
}

impl fmt::Display for TreeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            TreeState::Alive => "^",
            TreeState::Burning => "#",
            TreeState::Burned => "_",
        };
        write!(f, "{}", s)
    }
}

fn coord_ix(width: usize, c: Coord) -> usize {
    width * c.y + c.x
}

fn ix_coord(width: usize, c: usize) -> Coord {
    coord!(c / width, c % width)
}

fn neighbors(width: usize, height: usize, c: Coord) -> Vec<usize> {
    let w = width as isize;
    let h = height as isize;
    let xi = c.x as isize;
    let yi = c.y as isize;

    let mut v = Vec::with_capacity(8);
    let cs: Vec<(isize,isize)> =
        vec![(-1, -1), (0, -1), (1, -1),
             (-1,  0),          (1,  0),
             (-1,  1), (0,  1), (1,  1)];


    for (cx, cy) in cs {
        if 0 == xi && cx == -1 { continue };
        if w - 1 == xi && cx == 1 { continue };

        if 0 == yi && cy == -1 { continue };
        if h - 1 == yi && cy == 1 { continue };

        let nx = (c.x as isize + cx) as usize;
        let ny = (c.y as isize + cy) as usize;

        v.push(coord_ix(width, coord!(nx, ny)));
    }

    v
}

#[test]
fn test_forest_new() {
    let f = Forest::new(3,3);
    assert!(f.size() == 9);
    assert!(f.trees[0].state == TreeState::Alive);
}

#[test]
fn test_forest_light() {
    let mut f = Forest::new(3,3);
    f.light();
    assert!(TreeState::Burning == f.tree_at(coord!(1,1)).state);
}

#[test]
fn test_coord_ix() {
    let f = Forest::new(3,3);
    let c = ix_coord(f.width, 4);

    assert!(c.x == 1);
    assert!(c.y == 1);
}

#[test]
fn macro_works() {
    assert!(coord!(3,4).x == 3);
    assert!(coord!(3,4).y == 4);
}

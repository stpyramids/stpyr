#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Pos(pub u32, pub u32);
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct PosDiff(pub i32, pub i32);
pub type Bounds = (Pos, Pos);

pub fn clamp_xy<T: PartialOrd<T>>(xy: (T, T), low: (T, T), high: (T, T)) -> (T, T) {
    let (mut dx, mut dy) = xy;
    let (x0, y0) = low;
    let (x1, y1) = high;

    if dx < x0 {
        dx = x0
    }
    if dx > x1 {
        dx = x1
    }
    if dy < y0 {
        dy = y0
    }
    if dy > y1 {
        dy = y1
    }

    (dx, dy)
}

impl Pos {
    fn diff(&self, other: &Pos) -> PosDiff {
        PosDiff(
            self.0 as i32 - other.0 as i32,
            self.1 as i32 - other.1 as i32,
        )
    }
    pub fn clamp(&self, low: (u32, u32), high: (u32, u32)) -> Pos {
        let (x, y) = clamp_xy((self.0, self.1), low, high);
        Pos(x as u32, y as u32)
    }
    pub fn to_idx(&self, w: u32) -> usize {
        let Pos(x, y) = self;
        ((y * w as u32) + x) as usize
    }
}

impl PosDiff {
    pub fn clamp(&self, low: (i32, i32), high: (i32, i32)) -> PosDiff {
        let (x, y) = clamp_xy((self.0, self.1), low, high);
        PosDiff(x, y)
    }
}
pub trait HasPos {
    fn pos(&self) -> &Pos;
    fn clamp(&mut self, low: (u32, u32), high: (u32, u32)) {
        let pos = self.pos().to_owned();
        self.set_pos(pos.clamp(low, high));
    }
    fn set_pos(&mut self, pos: Pos);
    fn move_pos(&self, diff: PosDiff) -> Pos {
        self.move_pos_xy(diff.0, diff.1)
    }
    fn move_pos_xy(&self, dx: i32, dy: i32) -> Pos {
        let Pos(mut x, mut y) = self.pos();
        if dx >= 0 {
            x += dx as u32;
        } else {
            if x >= (-dx as u32) {
                x -= -dx as u32;
            } else {
                x = 0;
            }
        }
        if dy >= 0 {
            y += dy as u32;
        } else {
            if y >= (-dy as u32) {
                y -= -dy as u32;
            } else {
                y = 0;
            }
        }
        Pos(x, y)
    }
    fn pos_to_idx(&self, w: usize) -> usize {
        self.pos().to_idx(w as u32)
    }
    fn diff(&self, other: &HasPos) -> PosDiff {
        self.pos().diff(other.pos())
    }
}

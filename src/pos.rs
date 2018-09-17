#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Pos(pub u32, pub u32);
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct PosDiff(pub i32, pub i32);

impl Pos {
    fn diff(&self, other: &Pos) -> PosDiff {
        PosDiff(
            self.0 as i32 - other.0 as i32,
            self.1 as i32 - other.1 as i32,
        )
    }
}

pub trait HasPos {
    fn pos(&self) -> &Pos;
    fn set_pos(&mut self, pos: Pos);
    fn move_pos(&mut self, diff: PosDiff) {
        self.move_pos_xy(diff.0, diff.1);
    }
    fn move_pos_xy(&mut self, dx: i32, dy: i32) {
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
        self.set_pos(Pos(x, y));
    }
    fn pos_to_idx(&self, w: usize) -> usize {
        let Pos(x, y) = self.pos();
        ((y * w as u32) + x) as usize
    }
    fn diff(&self, other: &HasPos) -> PosDiff {
        self.pos().diff(other.pos())
    }
}

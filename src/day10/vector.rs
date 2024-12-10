#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Vector {
    pub x: usize,
    pub y: usize,
}

impl Vector {
    pub fn new(x: usize, y: usize) -> Self {
        Vector { x, y }
    }

    pub fn subtract(&self, other_x: usize, other_y: usize) -> Option<Vector> {
        self.x.checked_sub(other_x).and_then(|x| {
            self.y
                .checked_sub(other_y)
                .and_then(|y| Some(Vector { x, y }))
        })
    }

    pub fn add(&self, other_x: usize, other_y: usize) -> Self {
        Vector {
            x: self.x + other_x,
            y: self.y + other_y,
        }
    }

}

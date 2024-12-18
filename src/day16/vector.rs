#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Vector {
    pub x: usize,
    pub y: usize,
}

impl Vector {
    pub fn new(x: usize, y: usize) -> Self {
        Vector { x, y }
    }

    pub fn subtract(&self, x: usize, y: usize) -> Option<Vector> {
        self.x.checked_sub(x).and_then(|x| {
            self.y
                .checked_sub(y)
                .and_then(|y| Some(Vector { x, y }))
        })
    }

    pub fn add(&self, x: usize, y: usize) -> Self {
        Vector {
            x: self.x + x,
            y: self.y + y,
        }
    }

}

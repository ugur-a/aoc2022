#[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
pub struct Point2D<T>(pub T, pub T);

impl<T: Copy> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> T {
        self.0
    }

    pub fn y(&self) -> T {
        self.1
    }
}

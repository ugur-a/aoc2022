use std::ops::Add;

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

impl<T> Add for Point2D<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

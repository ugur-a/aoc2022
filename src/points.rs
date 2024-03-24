use std::ops::Add;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
pub struct Point2D<T, U = T>(pub T, pub U);

impl<T: Copy, U: Copy> Point2D<T, U> {
    pub fn new(x: T, y: U) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> T {
        self.0
    }

    pub fn y(&self) -> U {
        self.1
    }
}

impl<T, U> Add for Point2D<T, U>
where
    T: Add<Output = T>,
    U: Add<Output = U>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

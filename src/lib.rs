pub mod utils {
    use std::{fmt::Display, ops};

    #[derive(PartialEq, Eq, Hash, Clone)]
    pub struct Point<T: Ord> {
        pub x: T,
        pub y: T,
    }

    impl<T> ops::Add for Point<T>
    where
        T: Ord + Copy + ops::Add<Output = T>,
    {
        type Output = Point<T>;

        fn add(self, _rhs: Point<T>) -> Point<T> {
            return Point {
                x: self.x + _rhs.x,
                y: self.y + _rhs.y,
            };
        }
    }

    impl<T: Ord + Display> Display for Point<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}, {}", self.x, self.y)
        }
    }

    #[derive(PartialEq, Eq, Hash, Clone)]
    pub struct Rect<T: Ord> {
        pub x: T,
        pub y: T,
        pub width: T,
        pub height: T,
    }

    impl<T: Ord> Rect<T> {
        pub fn is_inside(&self, p: &Point<T>) -> bool {
            return p.x >= self.x && p.x < self.width && p.y >= self.y && p.y < self.height;
        }
    }
}

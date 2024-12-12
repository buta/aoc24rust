pub mod utils {
    use num::Signed;
    use std::{fmt::Display, ops};
    #[derive(PartialEq, Eq, Hash, Clone)]
    pub struct PointT<T: Ord> {
        pub x: T,
        pub y: T,
    }

    impl<T> ops::Add for PointT<T>
    where
        T: Ord + Copy + ops::Add<Output = T>,
    {
        type Output = PointT<T>;

        fn add(self, _rhs: PointT<T>) -> PointT<T> {
            return PointT {
                x: self.x + _rhs.x,
                y: self.y + _rhs.y,
            };
        }
    }

    impl<T> PointT<T>
    where
        T: Ord + Copy + ops::Sub + Signed,
    {
        pub fn distance(&self, other: &PointT<T>) -> T {
            return (self.x - other.x).abs() + (self.y - other.y).abs();
        }
    }

    impl<T: Ord + Display> Display for PointT<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}, {}", self.x, self.y)
        }
    }

    #[derive(PartialEq, Eq, Hash, Clone)]
    pub struct RectT<T: Ord> {
        pub x: T,
        pub y: T,
        pub width: T,
        pub height: T,
    }

    impl<T: Ord> RectT<T> {
        pub fn is_inside(&self, p: &PointT<T>) -> bool {
            return p.x >= self.x && p.x < self.width && p.y >= self.y && p.y < self.height;
        }
    }
}

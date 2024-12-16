pub mod utils {
    use num::{Integer, Signed};
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

    impl<T> ops::Mul<T> for PointT<T>
    where
        T: Ord + Copy + ops::Mul<Output = T>,
    {
        type Output = PointT<T>;

        fn mul(self, _rhs: T) -> PointT<T> {
            return PointT {
                x: self.x * _rhs,
                y: self.y * _rhs,
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

    impl<T> RectT<T>
    where
        T: Ord + Copy + ops::Add + ops::Div<T> + Signed + Integer,
    {
        pub fn contains_point(&self, p: &PointT<T>) -> bool {
            return p.x >= self.x
                && p.x < self.x + self.width
                && p.y >= self.y
                && p.y < self.y + self.height;
        }

        pub fn get_quadrants(&self) -> [RectT<T>; 4] {
            let two = T::one() + T::one();
            let half_width = self.width / two;
            let half_height = self.height / two;
            [
                RectT {
                    x: self.x,
                    y: self.y,
                    width: half_width,
                    height: half_height,
                },
                RectT {
                    x: self.width - half_width,
                    y: self.y,
                    width: half_width,
                    height: half_height,
                },
                RectT {
                    x: self.x,
                    y: self.height - half_height,
                    width: half_width,
                    height: half_height,
                },
                RectT {
                    x: self.width - half_width,
                    y: self.height - half_height,
                    width: half_width,
                    height: half_height,
                },
            ]
        }
    }
}

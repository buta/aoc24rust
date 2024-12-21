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

    impl<T> Ord for PointT<T>
    where
        T: Ord + Copy,
    {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.x.cmp(&other.x).then(self.y.cmp(&other.y))
        }
    }

    impl<T> PartialOrd for PointT<T>
    where
        T: Ord + Copy,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self.x.partial_cmp(&other.x) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
            self.y.partial_cmp(&other.y)
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

    impl<T> PointT<T>
    where
        T: Ord + Copy + ops::Sub + ops::AddAssign + ops::Add + Signed,
    {
        pub fn get_points_with_distance(&self, distance: T) -> Vec<PointT<T>> {
            // (coord)      =>  N    NE   E    SE   S    SW   W    NW
            // (2,2) dist 1 =>  2,3       3,2       2,1       1,2
            // (2,2) dist 2 =>  2,4  3,3  4,2  3,1  2,0  1,1  0,2  1,3
            let mut ret = Vec::new();
            let mut d = T::zero();
            loop {
                if d >= distance {
                    break;
                }
                ret.push(PointT {
                    x: self.x + d,
                    y: self.y + (distance - d),
                });
                ret.push(PointT {
                    x: self.x + (distance - d),
                    y: self.y - d,
                });
                ret.push(PointT {
                    x: self.x - d,
                    y: self.y - (distance - d),
                });
                ret.push(PointT {
                    x: self.x - (distance - d),
                    y: self.y + d,
                });
                d += T::one();
            }
            return ret;
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

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use utils::PointT;

    use super::*;
    #[test]
    fn test_point_generation() {
        let point = PointT { x: 2, y: 2 };
        let exp_1 = HashSet::from([
            PointT { x: 2, y: 3 },
            PointT { x: 3, y: 2 },
            PointT { x: 2, y: 1 },
            PointT { x: 1, y: 2 },
        ]);
        let dist_1 = point.get_points_with_distance(1);

        assert!(HashSet::from_iter(dist_1.into_iter()) == exp_1);

        let dist_2 = point.get_points_with_distance(2);
        let exp_2 = HashSet::from([
            PointT { x: 2, y: 4 },
            PointT { x: 3, y: 3 },
            PointT { x: 4, y: 2 },
            PointT { x: 3, y: 1 },
            PointT { x: 2, y: 0 },
            PointT { x: 1, y: 1 },
            PointT { x: 0, y: 2 },
            PointT { x: 1, y: 3 },
        ]);
        assert!(HashSet::from_iter(dist_2.into_iter()) == exp_2);
    }
}

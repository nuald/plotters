use std::ops::Range;

use super::{AsRangedCoord, DiscreteRanged, Ranged, ReversibleRanged};

macro_rules! impl_discrete_trait {
    ($name:ident) => {
        impl DiscreteRanged for $name {
            type RangeParameter = ();
            fn get_range_parameter(&self) -> () {}
            fn next_value(this: &Self::ValueType, _: &()) -> Self::ValueType {
                return *this + 1;
            }
            fn previous_value(this: &Self::ValueType, _: &()) -> Self::ValueType {
                return *this - 1;
            }
        }
    };
}

macro_rules! impl_ranged_type_trait {
    ($value:ty, $coord:ident) => {
        impl AsRangedCoord for Range<$value> {
            type CoordDescType = $coord;
            type Value = $value;
        }
    };
}

macro_rules! make_numeric_coord {
    ($type:ty, $name:ident, $key_points:ident, $doc: expr) => {
        #[doc = $doc]
        #[derive(Clone)]
        pub struct $name($type, $type);
        impl From<Range<$type>> for $name {
            fn from(range: Range<$type>) -> Self {
                return Self(range.start, range.end);
            }
        }
        impl Ranged for $name {
            type ValueType = $type;
            fn map(&self, v: &$type, limit: (i32, i32)) -> i32 {
                let logic_length = (*v - self.0) as f64 / (self.1 - self.0) as f64;
                let actual_length = limit.1 - limit.0;

                if actual_length == 0 {
                    return limit.1;
                }

                return limit.0 + (actual_length as f64 * logic_length + 1e-3).floor() as i32;
            }
            fn key_points(&self, max_points: usize) -> Vec<$type> {
                $key_points((self.0, self.1), max_points)
            }
            fn range(&self) -> Range<$type> {
                return self.0..self.1;
            }
        }

        impl ReversibleRanged for $name {
            fn unmap(&self, p:i32, (min,max): (i32, i32)) -> Option<$type> {
                if p < min.min(max) || p > max.max(min) {
                    return None;
                }

                let logical_offset = (p - min) as f64 / (max - min) as f64;

                return Some(((self.1 - self.0) as f64 * logical_offset + self.0 as f64) as $type);
            }
        }
    };
}

macro_rules! gen_key_points_comp {
    (float, $name:ident, $type:ty) => {
        fn $name(range: ($type, $type), max_points: usize) -> Vec<$type> {
            if max_points == 0 {
                return vec![];
            }

            let range = (range.0 as f64, range.1 as f64);
            let mut scale = (10f64).powf((range.1 - range.0).log(10.0).floor());
            let mut digits = -(range.1 - range.0).log(10.0).floor() as i32 + 1;
            fn rem_euclid(a: f64, b: f64) -> f64 {
                if b > 0.0 {
                    a - (a / b).floor() * b
                } else {
                    a - (a / b).ceil() * b
                }
            }

            // At this point we need to make sure that the loop invariant:
            // The scale must yield number of points than requested
            if 1 + ((range.1 - range.0) / scale).floor() as usize > max_points {
                scale *= 10.0;
            }

            'outer: loop {
                let old_scale = scale;
                for nxt in [2.0, 5.0, 10.0].iter() {
                    let new_left = range.0 + scale / nxt - rem_euclid(range.0, scale / nxt);
                    let new_right = range.1 - rem_euclid(range.1, scale / nxt);

                    let npoints = 1 + ((new_right - new_left) / old_scale * nxt) as usize;

                    if npoints > max_points {
                        break 'outer;
                    }

                    scale = old_scale / nxt;
                }
                scale = old_scale / 10.0;
                if scale < 1.0 {
                    digits += 1;
                }
            }

            let mut ret = vec![];
            let mut left = range.0 + scale - rem_euclid(range.0, scale);
            let right = range.1 - rem_euclid(range.1, scale);
            while left <= right {
                let size = (10f64).powf(digits as f64 + 1.0);
                let new_left = (left * size).abs() + 1e-3;
                if left < 0.0 {
                    left = -new_left.round() / size;
                } else {
                    left = new_left.round() / size;
                }
                ret.push(left as $type);
                left += scale;
            }
            return ret;
        }
    };
    (integer, $name:ident, $type:ty) => {
        fn $name(range: ($type, $type), max_points: usize) -> Vec<$type> {
            let mut scale: $type = 1;
            let range = (range.0.min(range.1), range.0.max(range.1));
            'outer: while (range.1 - range.0 + scale - 1) as usize / (scale as usize) > max_points {
                let next_scale = scale * 10;
                for new_scale in [scale * 2, scale * 5, scale * 10].iter() {
                    scale = *new_scale;
                    if (range.1 - range.0 + *new_scale - 1) as usize / (*new_scale as usize)
                        < max_points
                    {
                        break 'outer;
                    }
                }
                scale = next_scale;
            }

            let (mut left, right) = (
                range.0 + (scale - range.0 % scale) % scale,
                range.1 - range.1 % scale,
            );

            let mut ret = vec![];
            while left <= right {
                ret.push(left as $type);
                left += scale;
            }

            return ret;
        }
    };
}

gen_key_points_comp!(float, compute_f32_key_points, f32);
gen_key_points_comp!(float, compute_f64_key_points, f64);
gen_key_points_comp!(integer, compute_i32_key_points, i32);
gen_key_points_comp!(integer, compute_u32_key_points, u32);
gen_key_points_comp!(integer, compute_i64_key_points, i64);
gen_key_points_comp!(integer, compute_u64_key_points, u64);
gen_key_points_comp!(integer, compute_i128_key_points, i128);
gen_key_points_comp!(integer, compute_u128_key_points, u128);
gen_key_points_comp!(integer, compute_isize_key_points, isize);
gen_key_points_comp!(integer, compute_usize_key_points, usize);

make_numeric_coord!(
    f32,
    RangedCoordf32,
    compute_f32_key_points,
    "The ranged coordinate for type f32"
);
make_numeric_coord!(
    f64,
    RangedCoordf64,
    compute_f64_key_points,
    "The ranged coordinate for type f64"
);
make_numeric_coord!(
    u32,
    RangedCoordu32,
    compute_u32_key_points,
    "The ranged coordinate for type u32"
);
make_numeric_coord!(
    i32,
    RangedCoordi32,
    compute_i32_key_points,
    "The ranged coordinate for type i32"
);
make_numeric_coord!(
    u64,
    RangedCoordu64,
    compute_u64_key_points,
    "The ranged coordinate for type u64"
);
make_numeric_coord!(
    i64,
    RangedCoordi64,
    compute_i64_key_points,
    "The ranged coordinate for type i64"
);
make_numeric_coord!(
    u128,
    RangedCoordu128,
    compute_u128_key_points,
    "The ranged coordinate for type u128"
);
make_numeric_coord!(
    i128,
    RangedCoordi128,
    compute_i128_key_points,
    "The ranged coordinate for type i128"
);
make_numeric_coord!(
    usize,
    RangedCoordusize,
    compute_usize_key_points,
    "The ranged coordinate for type usize"
);
make_numeric_coord!(
    isize,
    RangedCoordisize,
    compute_isize_key_points,
    "The ranged coordinate for type isize"
);

impl_discrete_trait!(RangedCoordu32);
impl_discrete_trait!(RangedCoordi32);
impl_discrete_trait!(RangedCoordu64);
impl_discrete_trait!(RangedCoordi64);
impl_discrete_trait!(RangedCoordu128);
impl_discrete_trait!(RangedCoordi128);
impl_discrete_trait!(RangedCoordusize);
impl_discrete_trait!(RangedCoordisize);

impl_ranged_type_trait!(f32, RangedCoordf32);
impl_ranged_type_trait!(f64, RangedCoordf64);
impl_ranged_type_trait!(i32, RangedCoordi32);
impl_ranged_type_trait!(u32, RangedCoordu32);
impl_ranged_type_trait!(i64, RangedCoordi64);
impl_ranged_type_trait!(u64, RangedCoordu64);
impl_ranged_type_trait!(i128, RangedCoordi128);
impl_ranged_type_trait!(u128, RangedCoordu128);
impl_ranged_type_trait!(isize, RangedCoordisize);
impl_ranged_type_trait!(usize, RangedCoordusize);

// TODO: Think about how to re-organize this part
pub mod group_integer_by {
    use super::Ranged;
    use super::{AsRangedCoord, DiscreteRanged};
    use num_traits::{FromPrimitive, PrimInt, ToPrimitive};
    use std::ops::{Mul, Range};

    /// The ranged value spec that needs to be grouped.
    /// This is useful, for example, when we have an X axis is a integer and denotes days.
    /// And we are expecting the tick mark denotes weeks, in this way we can make the range
    /// spec grouping by 7 elements.
    pub struct GroupBy<T>(T, T::ValueType)
    where
        T::ValueType: PrimInt + ToPrimitive + FromPrimitive + Mul,
        T: Ranged;

    /// The trait that provides method `Self::group_by` function which creates a
    /// `GroupBy` decorated ranged value.
    pub trait ToGroupByRange
    where
        Self: AsRangedCoord,
        <Self as AsRangedCoord>::Value: PrimInt + ToPrimitive + FromPrimitive + Mul,
        <<Self as AsRangedCoord>::CoordDescType as Ranged>::ValueType:
            PrimInt + ToPrimitive + FromPrimitive + Mul,
    {
        /// Make a grouping ranged value, see the documentation for `GroupBy` for details.
        ///
        /// - `value`: The number of values we want to group it
        /// - **return**: The newly created grouping range sepcification
        fn group_by(
            self,
            value: <<Self as AsRangedCoord>::CoordDescType as Ranged>::ValueType,
        ) -> GroupBy<<Self as AsRangedCoord>::CoordDescType> {
            GroupBy(self.into(), value)
        }
    }

    impl<T> ToGroupByRange for T
    where
        Self: AsRangedCoord,
        <Self as AsRangedCoord>::Value: PrimInt + FromPrimitive + ToPrimitive + Mul,
        <<Self as AsRangedCoord>::CoordDescType as Ranged>::ValueType:
            PrimInt + FromPrimitive + ToPrimitive + Mul,
    {
    }

    impl<T> AsRangedCoord for GroupBy<T>
    where
        T::ValueType: PrimInt + ToPrimitive + FromPrimitive + Mul,
        T: Ranged,
    {
        type Value = T::ValueType;
        type CoordDescType = Self;
    }

    impl<T> DiscreteRanged for GroupBy<T>
    where
        T::ValueType: PrimInt + ToPrimitive + FromPrimitive + Mul,
        T: Ranged + DiscreteRanged,
    {
        type RangeParameter = <T as DiscreteRanged>::RangeParameter;
        fn get_range_parameter(&self) -> Self::RangeParameter {
            self.0.get_range_parameter()
        }
        fn previous_value(this: &Self::ValueType, param: &Self::RangeParameter) -> Self::ValueType {
            <T as DiscreteRanged>::previous_value(this, param)
        }
        fn next_value(this: &Self::ValueType, param: &Self::RangeParameter) -> Self::ValueType {
            <T as DiscreteRanged>::next_value(this, param)
        }
    }

    impl<T> Ranged for GroupBy<T>
    where
        T::ValueType: PrimInt + ToPrimitive + FromPrimitive + Mul,
        T: Ranged,
    {
        type ValueType = T::ValueType;
        fn map(&self, value: &T::ValueType, limit: (i32, i32)) -> i32 {
            self.0.map(value, limit)
        }
        fn range(&self) -> Range<T::ValueType> {
            self.0.range()
        }
        fn key_points(&self, max_points: usize) -> Vec<T::ValueType> {
            let actual_range = self.0.range();
            let from = ((actual_range.start + self.1 - T::ValueType::from_u8(1).unwrap()) / self.1)
                .to_isize()
                .unwrap();
            let to = (actual_range.end / self.1).to_isize().unwrap();
            let logic_range: super::RangedCoordisize = (from..to).into();

            logic_range
                .key_points(max_points)
                .into_iter()
                .map(|x| T::ValueType::from_isize(x).unwrap() * self.1)
                .collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::coord::*;
    #[test]
    fn test_key_points() {
        let kp = compute_i32_key_points((0, 999), 28);

        assert!(kp.len() > 0);
        assert!(kp.len() <= 28);

        let kp = compute_f64_key_points((-1.2, 1.2), 1);
        assert!(kp.len() == 1);

        let kp = compute_f64_key_points((-1.2, 1.2), 0);
        assert!(kp.len() == 0);
    }

    #[test]
    fn test_linear_coord_map() {
        let coord: RangedCoordu32 = (0..20).into();
        assert_eq!(coord.key_points(11).len(), 11);
        assert_eq!(coord.key_points(11)[0], 0);
        assert_eq!(coord.key_points(11)[10], 20);
        assert_eq!(coord.map(&5, (0, 100)), 25);

        let coord: RangedCoordf32 = (0f32..20f32).into();
        assert_eq!(coord.map(&5.0, (0, 100)), 25);
    }

    #[test]
    fn test_linear_coord_system() {
        let _coord =
            RangedCoord::<RangedCoordu32, RangedCoordu32>::new(0..10, 0..10, (0..1024, 0..768));
    }
}

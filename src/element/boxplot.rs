/*!
  The boxplot element, which showing the quartiles
*/
use std::marker::PhantomData;

use crate::drawing::backend::{BackendCoord, DrawingBackend, DrawingErrorKind};
use crate::element::{Drawable, PointCollection};
use crate::style::{ShapeStyle, GREEN};

fn median<T: Into<f64> + Copy + PartialOrd>(s: &[T]) -> f64 {
    let mut s = s.to_owned();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    match s.len() % 2 {
        0 => (s[(s.len() / 2) - 1].into() / 2.0) + (s[(s.len() / 2)].into() / 2.0),
        _ => s[s.len() / 2].into(),
    }
}

fn quartiles<T: Into<f64> + Copy + PartialOrd>(s: &[T]) -> (f64, f64, f64) {
    if s.len() == 1 {
        let value = s[0].into();
        return (value, value, value);
    }
    let mut s = s.to_owned();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let (a, b) = if s.len() % 2 == 0 {
        s.split_at(s.len() / 2)
    } else {
        (&s[..(s.len() / 2)], &s[((s.len() / 2) + 1)..])
    };
    (median(a), median(&s), median(b))
}

fn values<T: Into<f64> + Copy + PartialOrd>(s: &[T]) -> [f32; 5] {
    let (q1, q2, q3) = quartiles(s);
    let iqr = q3 - q1;
    [
        (q1 - 1.5 * iqr) as f32,
        q1 as f32,
        q2 as f32,
        q3 as f32,
        (q3 + 1.5 * iqr) as f32,
    ]
}

pub trait BoxplotOrient<K, V> {
    type XType;
    type YType;

    fn make_coord(key: K, val: V) -> (Self::XType, Self::YType);
    fn with_offset(coord: BackendCoord, offset: f64) -> BackendCoord;
}

pub struct BoxplotOrientV<K, V>(PhantomData<(K, V)>);

pub struct BoxplotOrientH<K, V>(PhantomData<(K, V)>);

impl<K, V> BoxplotOrient<K, V> for BoxplotOrientV<K, V> {
    type XType = K;
    type YType = V;

    fn make_coord(key: K, val: V) -> (K, V) {
        (key, val)
    }

    fn with_offset(coord: BackendCoord, offset: f64) -> BackendCoord {
        (coord.0 + offset as i32, coord.1)
    }
}

impl<K, V> BoxplotOrient<K, V> for BoxplotOrientH<K, V> {
    type XType = V;
    type YType = K;

    fn make_coord(key: K, val: V) -> (V, K) {
        (val, key)
    }

    fn with_offset(coord: BackendCoord, offset: f64) -> BackendCoord {
        (coord.0, coord.1 + offset as i32)
    }
}

/// The boxplot data point element
pub struct Boxplot<K, O: BoxplotOrient<K, f32>> {
    style: ShapeStyle,
    width: u32,
    whisker_width: f64,
    offset: f64,
    key: K,
    values: [f32; 5],
    _p: PhantomData<O>,
}

impl<K: Clone> Boxplot<K, BoxplotOrientV<K, f32>> {
    /// Create a new vertical boxplot element
    ///
    /// ```rust
    /// use plotters::element::{Boxplot, PointCollection};
    ///
    /// let plot = Boxplot::new_vertical("group", &[7, 15, 36, 39, 40, 41]);
    /// let points = &plot.point_iter()[1..4];
    /// assert_eq!(points[0].1, 15.0, "lower quartile");
    /// assert_eq!(points[1].1, 37.5, "median");
    /// assert_eq!(points[2].1, 40.0, "upper quartile");
    /// ```
    pub fn new_vertical<T>(key: K, data: &[T]) -> Self
    where
        T: Into<f64> + Copy + PartialOrd,
    {
        Self {
            style: Into::<ShapeStyle>::into(&GREEN),
            width: 5,
            whisker_width: 1.0,
            offset: 0.0,
            key,
            values: values(data),
            _p: PhantomData,
        }
    }
}

impl<K: Clone> Boxplot<K, BoxplotOrientH<K, f32>> {
    /// Create a new horizontal boxplot element
    ///
    /// ```rust
    /// use plotters::element::{Boxplot, PointCollection};
    ///
    /// let plot = Boxplot::new_horizontal("group", &[7, 15, 36, 39, 40, 41]);
    /// let points = &plot.point_iter()[1..4];
    /// assert_eq!(points[0].0, 15.0, "lower quartile");
    /// assert_eq!(points[1].0, 37.5, "median");
    /// assert_eq!(points[2].0, 40.0, "upper quartile");
    /// ```
    pub fn new_horizontal<T>(key: K, data: &[T]) -> Self
    where
        T: Into<f64> + Copy + PartialOrd,
    {
        Self {
            style: Into::<ShapeStyle>::into(&GREEN),
            width: 5,
            whisker_width: 1.0,
            offset: 0.0,
            key,
            values: values(data),
            _p: PhantomData,
        }
    }
}

impl<K, O: BoxplotOrient<K, f32>> Boxplot<K, O> {
    /// Set the style of the boxplot
    pub fn style<S: Into<ShapeStyle>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Set the bar width
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Set the width of the whiskers as a fraction of the bar width
    pub fn whisker_width(mut self, whisker_width: f64) -> Self {
        self.whisker_width = whisker_width;
        self
    }

    /// Set the element offset
    pub fn offset<T: Into<f64> + Copy>(mut self, offset: T) -> Self {
        self.offset = offset.into();
        self
    }
}

impl<'a, K: 'a + Clone, O: BoxplotOrient<K, f32>> PointCollection<'a, (O::XType, O::YType)>
    for &'a Boxplot<K, O>
{
    type Borrow = (O::XType, O::YType);
    type IntoIter = Vec<Self::Borrow>;
    fn point_iter(self) -> Self::IntoIter {
        self.values
            .iter()
            .map(|v| O::make_coord(self.key.clone(), *v))
            .collect()
    }
}

impl<K, DB: DrawingBackend, O: BoxplotOrient<K, f32>> Drawable<DB> for Boxplot<K, O> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let points: Vec<_> = points.take(5).collect();
        if points.len() == 5 {
            let width = f64::from(self.width);
            let moved = |coord| O::with_offset(coord, self.offset);
            let start_bar = |coord| O::with_offset(moved(coord), -width / 2.0);
            let end_bar = |coord| O::with_offset(moved(coord), width / 2.0);
            let start_whisker =
                |coord| O::with_offset(moved(coord), -width * self.whisker_width / 2.0);
            let end_whisker =
                |coord| O::with_offset(moved(coord), width * self.whisker_width / 2.0);

            // |---[   |  ]----|
            // ^________________
            backend.draw_line(
                start_whisker(points[0]),
                end_whisker(points[0]),
                &self.style.color,
            )?;

            // |---[   |  ]----|
            // _^^^_____________
            backend.draw_line(moved(points[0]), moved(points[1]), &self.style.color)?;

            // |---[   |  ]----|
            // ____^______^_____
            let corner1 = start_bar(points[3]);
            let corner2 = end_bar(points[1]);
            let upper_left = (corner1.0.min(corner2.0), corner1.1.min(corner2.1));
            let bottom_right = (corner1.0.max(corner2.0), corner1.1.max(corner2.1));
            backend.draw_rect(upper_left, bottom_right, &self.style.color, false)?;

            // |---[   |  ]----|
            // ________^________
            backend.draw_line(start_bar(points[2]), end_bar(points[2]), &self.style.color)?;

            // |---[   |  ]----|
            // ____________^^^^_
            backend.draw_line(moved(points[3]), moved(points[4]), &self.style.color)?;

            // |---[   |  ]----|
            // ________________^
            backend.draw_line(
                start_whisker(points[4]),
                end_whisker(points[4]),
                &self.style.color,
            )?;
        }
        Ok(())
    }
}

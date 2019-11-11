/// The category coordinates
use std::fmt;
use std::ops::Range;
use std::rc::Rc;

use super::{AsRangedCoord, Ranged};

pub struct Category<T: PartialEq> {
    name: String,
    elements: Rc<Vec<T>>,
}

pub struct CategoryElementRef<T: PartialEq> {
    inner: Rc<Vec<T>>,
    // i32 type is required for the empty ref (having -1 value)
    idx: i32,
}

pub struct CategoryElementsRange<T: PartialEq>(CategoryElementRef<T>, CategoryElementRef<T>);

impl<T: PartialEq> Clone for CategoryElementRef<T> {
    fn clone(&self) -> Self {
        CategoryElementRef {
            inner: Rc::clone(&self.inner),
            idx: self.idx,
        }
    }
}

impl<T: PartialEq + fmt::Display> fmt::Debug for CategoryElementRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let element = &self.inner[self.idx as usize];
        write!(f, "{}", element)
    }
}

impl<T: PartialEq> Category<T> {
    pub fn new<S: Into<String>>(name: S, elements: Vec<T>) -> Self {
        Self {
            name: name.into(),
            elements: Rc::new(elements),
        }
    }

    pub fn get(&self, val: &T) -> Option<CategoryElementRef<T>> {
        match self.elements.iter().position(|x| x == val) {
            Some(pos) => {
                let element_ref = CategoryElementRef {
                    inner: Rc::clone(&self.elements),
                    idx: pos as i32,
                };
                Some(element_ref)
            }
            _ => None,
        }
    }

    pub fn range(&self) -> CategoryElementsRange<T> {
        let start = 0;
        let end = self.elements.len() as i32 - 1;
        CategoryElementsRange(
            CategoryElementRef {
                inner: Rc::clone(&self.elements),
                idx: start,
            },
            CategoryElementRef {
                inner: Rc::clone(&self.elements),
                idx: end,
            },
        )
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl<T: PartialEq> From<Range<CategoryElementRef<T>>> for CategoryElementsRange<T> {
    fn from(range: Range<CategoryElementRef<T>>) -> Self {
        Self(range.start, range.end)
    }
}

impl<T: PartialEq> Ranged for CategoryElementsRange<T> {
    type ValueType = CategoryElementRef<T>;

    fn range(&self) -> Range<CategoryElementRef<T>> {
        self.0.clone()..self.1.clone()
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        // Add margins to spans as edge values are not applicable to category
        let total_span = (self.1.idx - self.0.idx + 2) as f64;
        let value_span = (value.idx - self.0.idx + 1) as f64;
        (f64::from(limit.1 - limit.0) * value_span / total_span) as i32 + limit.0
    }

    fn key_points(&self, max_points: usize) -> Vec<Self::ValueType> {
        let mut ret = vec![];
        let intervals = (self.1.idx - self.0.idx) as f64;
        let inner = &self.0.inner;
        let step = (intervals / max_points as f64 + 1.0) as usize;
        for idx in (self.0.idx..=self.1.idx).step_by(step) {
            ret.push(CategoryElementRef {
                inner: Rc::clone(&inner),
                idx,
            });
        }
        ret
    }
}

impl<T: PartialEq> AsRangedCoord for Range<CategoryElementRef<T>> {
    type CoordDescType = CategoryElementsRange<T>;
    type Value = CategoryElementRef<T>;
}

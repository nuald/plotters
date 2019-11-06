/*!
  The quartiles
*/

/// The quartiles
pub struct Quartiles {
    lower_fence: f64,
    lower: f64,
    median: f64,
    upper: f64,
    upper_fence: f64,
}

impl Quartiles {
    fn calc_median<T: Into<f64> + Copy + PartialOrd>(s: &[T]) -> f64 {
        let mut s = s.to_owned();
        s.sort_by(|a, b| a.partial_cmp(b).unwrap());
        match s.len() % 2 {
            0 => (s[(s.len() / 2) - 1].into() / 2.0) + (s[(s.len() / 2)].into() / 2.0),
            _ => s[s.len() / 2].into(),
        }
    }

    pub fn new<T: Into<f64> + Copy + PartialOrd>(s: &[T]) -> Self {
        if s.len() == 1 {
            let value = s[0].into();
            return Self {
                lower_fence: value,
                lower: value,
                median: value,
                upper: value,
                upper_fence: value,
            };
        }
        let mut s = s.to_owned();
        s.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let (a, b) = if s.len() % 2 == 0 {
            s.split_at(s.len() / 2)
        } else {
            (&s[..(s.len() / 2)], &s[((s.len() / 2) + 1)..])
        };
        let lower = Quartiles::calc_median(a);
        let median = Quartiles::calc_median(&s);
        let upper = Quartiles::calc_median(b);
        let iqr = upper - lower;
        let lower_fence = lower - 1.5 * iqr;
        let upper_fence = upper + 1.5 * iqr;
        Self {
            lower_fence,
            lower,
            median,
            upper,
            upper_fence,
        }
    }

    pub fn values(&self) -> [f32; 5] {
        [
            self.lower_fence as f32,
            self.lower as f32,
            self.median as f32,
            self.upper as f32,
            self.upper_fence as f32,
        ]
    }
}

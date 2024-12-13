pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

#[test]
fn test_size() {
    let interval = Interval::new(1.0, 5.0);
    assert_eq!(interval.size(), 4.0);
}

#[test]
fn test_contains() {
    let interval = Interval::new(1.0, 5.0);
    assert!(interval.contains(3.0));
    assert!(!interval.contains(0.0));
    assert!(!interval.contains(6.0));
}

#[test]
fn test_surrounds() {
    let interval = Interval::new(1.0, 5.0);
    assert!(interval.surrounds(3.0));
    assert!(!interval.surrounds(1.0));
    assert!(!interval.surrounds(5.0));
}

#[test]
fn test_clamp() {
    let interval = Interval::new(1.0, 5.0);
    assert_eq!(interval.clamp(0.0), 1.0);
    assert_eq!(interval.clamp(3.0), 3.0);
    assert_eq!(interval.clamp(6.0), 5.0);
}

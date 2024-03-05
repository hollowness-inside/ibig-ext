use ibig::{ubig, UBig};

/// Defines the `sqrt` function, which attempts to compute the square root of the implementor type.
pub trait BinarySqrt {
    /// Attempts to compute the square root of `self` using a binary search algorithm.
    /// Returns `Some(root)` if a square root exists, `None` otherwise.
    fn sqrt(&self) -> Option<Self>
    where
        Self: Sized;
}

impl BinarySqrt for UBig {
    fn sqrt(&self) -> Option<UBig> {
        if self <= &ubig!(1) {
            return Some(self.clone());
        }

        let mut low = ubig!(2);
        let mut high = self.clone();

        while low <= high {
            let mid: UBig = (&low + &high) / 2;
            let square = &mid * &mid;

            match square.cmp(self) {
                std::cmp::Ordering::Less => low = &mid + 1,
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Greater => high = &mid - 1,
            }
        }

        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinateError {
    Overflow,
    Underflow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub const fn position(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn apply_delta(value: u32, delta: i32) -> Result<u32, CoordinateError> {
        if delta < 0 {
            value
                .checked_sub(delta.unsigned_abs())
                .ok_or(CoordinateError::Underflow)
        } else {
            value
                .checked_add(delta as u32)
                .ok_or(CoordinateError::Overflow)
        }
    }

    pub fn add_to_coordinate(&mut self, x: i32, y: i32) -> Result<(), CoordinateError> {
        let changed_x = Self::apply_delta(self.x, x)?;
        let changed_y = Self::apply_delta(self.y, y)?;

        self.x = changed_x;
        self.y = changed_y;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_reads_coordinates() {
        let coordinate = Coordinate::new(0, 0);

        assert_eq!(coordinate.position(), (0, 0));
    }

    #[test]
    fn rejects_underflow_without_changing_coordinate() {
        let mut coordinate = Coordinate::new(5, 5);

        let result = coordinate.add_to_coordinate(-6, 2);
        assert_eq!(result, Err(CoordinateError::Underflow));
        assert_eq!(coordinate.position(), (5, 5));

        let result = coordinate.add_to_coordinate(-1, -6);
        assert_eq!(result, Err(CoordinateError::Underflow));
        assert_eq!(coordinate.position(), (5, 5));

        let result = coordinate.add_to_coordinate(-6, -6);
        assert_eq!(result, Err(CoordinateError::Underflow));
        assert_eq!(coordinate.position(), (5, 5));
    }

    #[test]
    fn rejects_i32_min_negative_delta_as_underflow() {
        let mut coordinate = Coordinate::new(0, 0);

        let result = coordinate.add_to_coordinate(i32::MIN, 0);

        assert_eq!(result, Err(CoordinateError::Underflow));
        assert_eq!(coordinate.position(), (0, 0));
    }

    #[test]
    fn rejects_y_error_without_changing_x() {
        let mut coordinate = Coordinate::new(10, 10);

        let result = coordinate.add_to_coordinate(1, -11);

        assert_eq!(result, Err(CoordinateError::Underflow));
        assert_eq!(coordinate.position(), (10, 10));
    }

    #[test]
    fn rejects_overflow_without_changing_coordinate() {
        let max_minus_5 = u32::MAX - 5;

        let mut coordinate = Coordinate::new(max_minus_5, max_minus_5);

        let result = coordinate.add_to_coordinate(6, 2);
        assert_eq!(result, Err(CoordinateError::Overflow));
        assert_eq!(coordinate.position(), (max_minus_5, max_minus_5));

        let result = coordinate.add_to_coordinate(-1, 6);
        assert_eq!(result, Err(CoordinateError::Overflow));
        assert_eq!(coordinate.position(), (max_minus_5, max_minus_5));

        let result = coordinate.add_to_coordinate(6, 6);
        assert_eq!(result, Err(CoordinateError::Overflow));
        assert_eq!(coordinate.position(), (max_minus_5, max_minus_5));
    }

    #[test]
    fn add_zero_coordinate_keeps_coordinate_unchanged() {
        let mut coordinate = Coordinate::new(10, 5);

        let result = coordinate.add_to_coordinate(0, 0);
        assert_eq!(result, Ok(()));
        assert_eq!(coordinate.position(), (10, 5));

        let result = coordinate.add_to_coordinate(-0, -0);
        assert_eq!(result, Ok(()));
        assert_eq!(coordinate.position(), (10, 5));
    }

    #[test]
    fn add_coordinate_to_coordinate() {
        let mut coordinate = Coordinate::new(10, 5);

        let result = coordinate.add_to_coordinate(1, 0);
        assert_eq!(result, Ok(()));
        assert_eq!(coordinate.position(), (11, 5));

        let result = coordinate.add_to_coordinate(0, 1);
        assert_eq!(result, Ok(()));
        assert_eq!(coordinate.position(), (11, 6));

        let result = coordinate.add_to_coordinate(2, 3);
        assert_eq!(result, Ok(()));
        assert_eq!(coordinate.position(), (13, 9));
    }

    #[test]
    fn sub_coordinate_to_coordinate() {
        let mut coordinate = Coordinate::new(10, 5);

        let result = coordinate.add_to_coordinate(-1, 0);
        assert_eq!(result, Ok(()));
        assert_eq!(coordinate.position(), (9, 5));

        let result = coordinate.add_to_coordinate(0, -1);
        assert_eq!(result, Ok(()));
        assert_eq!(coordinate.position(), (9, 4));

        let result = coordinate.add_to_coordinate(-2, -3);
        assert_eq!(result, Ok(()));
        assert_eq!(coordinate.position(), (7, 1));
    }

    #[test]
    fn add_sub_coordinate_to_coordinate() {
        let mut coordinate = Coordinate::new(10, 5);

        let result = coordinate.add_to_coordinate(-1, 1);
        assert_eq!(result, Ok(()));
        assert_eq!(coordinate.position(), (9, 6));

        let result = coordinate.add_to_coordinate(5, -5);
        assert_eq!(result, Ok(()));
        assert_eq!(coordinate.position(), (14, 1));

        let result = coordinate.add_to_coordinate(-4, 15);
        assert_eq!(result, Ok(()));
        assert_eq!(coordinate.position(), (10, 16));
    }
}

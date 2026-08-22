#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorldTimeError {
    Overflow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorldTime {
    milliseconds: u64,
}

const MILLISECONDS_PER_SECOND: u64 = 1_000;
const MILLISECONDS_PER_MINUTE: u64 = 60 * MILLISECONDS_PER_SECOND;
const MILLISECONDS_PER_HOUR: u64 = 60 * MILLISECONDS_PER_MINUTE;
const MILLISECONDS_PER_DAY: u64 = 24 * MILLISECONDS_PER_HOUR;

impl WorldTime {
    pub const fn from_millis(milliseconds: u64) -> Self {
        Self { milliseconds }
    }

    pub const fn as_millis(self) -> u64 {
        self.milliseconds
    }

    pub fn add_millis(&mut self, milliseconds: u64) -> Result<(), WorldTimeError> {
        self.milliseconds = self
            .milliseconds
            .checked_add(milliseconds)
            .ok_or(WorldTimeError::Overflow)?;

        Ok(())
    }

    pub fn add_seconds(&mut self, seconds: u64) -> Result<(), WorldTimeError> {
        let milliseconds = seconds
            .checked_mul(MILLISECONDS_PER_SECOND)
            .ok_or(WorldTimeError::Overflow)?;

        self.add_millis(milliseconds)
    }

    pub fn add_second(&mut self) -> Result<(), WorldTimeError> {
        self.add_seconds(1)
    }

    pub fn add_minutes(&mut self, minutes: u64) -> Result<(), WorldTimeError> {
        let milliseconds = minutes
            .checked_mul(MILLISECONDS_PER_MINUTE)
            .ok_or(WorldTimeError::Overflow)?;

        self.add_millis(milliseconds)
    }

    pub fn add_minute(&mut self) -> Result<(), WorldTimeError> {
        self.add_minutes(1)
    }

    pub fn add_hours(&mut self, hours: u64) -> Result<(), WorldTimeError> {
        let milliseconds = hours
            .checked_mul(MILLISECONDS_PER_HOUR)
            .ok_or(WorldTimeError::Overflow)?;

        self.add_millis(milliseconds)
    }

    pub fn add_hour(&mut self) -> Result<(), WorldTimeError> {
        self.add_hours(1)
    }

    pub fn add_days(&mut self, days: u64) -> Result<(), WorldTimeError> {
        let milliseconds = days
            .checked_mul(MILLISECONDS_PER_DAY)
            .ok_or(WorldTimeError::Overflow)?;

        self.add_millis(milliseconds)
    }

    pub fn add_day(&mut self) -> Result<(), WorldTimeError> {
        self.add_days(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_reads_world_time() {
        let time = WorldTime::from_millis(42);

        assert_eq!(time.as_millis(), 42);
    }

    #[test]
    fn adds_milliseconds() {
        let mut time = WorldTime::from_millis(100);

        let result = time.add_millis(25);

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), 125);
    }

    #[test]
    fn rejects_overflow_without_changing_time() {
        let mut time = WorldTime::from_millis(u64::MAX - 5);

        let result = time.add_millis(6);

        assert_eq!(result, Err(WorldTimeError::Overflow));
        assert_eq!(time.as_millis(), u64::MAX - 5);
    }

    #[test]
    fn adds_one_second() {
        let mut time = WorldTime::from_millis(0);

        let result = time.add_second();

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), MILLISECONDS_PER_SECOND);
    }

    #[test]
    fn adds_multiple_seconds() {
        let mut time = WorldTime::from_millis(1_000);

        let result = time.add_seconds(3);

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), 1_000 + MILLISECONDS_PER_SECOND * 3);
    }

    #[test]
    fn adding_zero_seconds_keeps_time_unchanged() {
        let mut time = WorldTime::from_millis(42);

        let result = time.add_seconds(0);

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), 42);
    }

    #[test]
    fn rejects_seconds_conversion_overflow() {
        let mut time = WorldTime::from_millis(0);

        let result = time.add_seconds(u64::MAX);

        assert_eq!(result, Err(WorldTimeError::Overflow));
        assert_eq!(time.as_millis(), 0);
    }

    #[test]
    fn adds_one_minute() {
        let mut time = WorldTime::from_millis(0);

        let result = time.add_minute();

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), MILLISECONDS_PER_MINUTE);
    }

    #[test]
    fn adds_multiple_minutes() {
        let mut time = WorldTime::from_millis(1_000);

        let result = time.add_minutes(3);

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), 1_000 + MILLISECONDS_PER_MINUTE * 3);
    }

    #[test]
    fn adding_zero_minutes_keeps_time_unchanged() {
        let mut time = WorldTime::from_millis(42);

        let result = time.add_minutes(0);

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), 42);
    }

    #[test]
    fn rejects_minutes_conversion_overflow() {
        let mut time = WorldTime::from_millis(0);

        let result = time.add_minutes(u64::MAX);

        assert_eq!(result, Err(WorldTimeError::Overflow));
        assert_eq!(time.as_millis(), 0);
    }

    #[test]
    fn adds_one_hour() {
        let mut time = WorldTime::from_millis(0);

        let result = time.add_hour();

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), MILLISECONDS_PER_HOUR);
    }

    #[test]
    fn adds_multiple_hours() {
        let mut time = WorldTime::from_millis(1_000);

        let result = time.add_hours(3);

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), 1_000 + MILLISECONDS_PER_HOUR * 3);
    }

    #[test]
    fn adding_zero_hours_keeps_time_unchanged() {
        let mut time = WorldTime::from_millis(42);

        let result = time.add_hours(0);

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), 42);
    }

    #[test]
    fn rejects_hours_conversion_overflow() {
        let mut time = WorldTime::from_millis(0);

        let result = time.add_hours(u64::MAX);

        assert_eq!(result, Err(WorldTimeError::Overflow));
        assert_eq!(time.as_millis(), 0);
    }

    #[test]
    fn adds_one_day() {
        let mut time = WorldTime::from_millis(0);

        let result = time.add_day();

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), MILLISECONDS_PER_DAY);
    }

    #[test]
    fn adds_multiple_days() {
        let mut time = WorldTime::from_millis(1_000);

        let result = time.add_days(3);

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), 1_000 + MILLISECONDS_PER_DAY * 3);
    }

    #[test]
    fn adding_zero_days_keeps_time_unchanged() {
        let mut time = WorldTime::from_millis(42);

        let result = time.add_days(0);

        assert_eq!(result, Ok(()));
        assert_eq!(time.as_millis(), 42);
    }

    #[test]
    fn rejects_day_conversion_overflow() {
        let mut time = WorldTime::from_millis(0);

        let result = time.add_days(u64::MAX);

        assert_eq!(result, Err(WorldTimeError::Overflow));
        assert_eq!(time.as_millis(), 0);
    }
}

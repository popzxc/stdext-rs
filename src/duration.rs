use std::time::Duration;

const SECS_IN_MIN: u64 = 60;
const SECS_IN_HOUR: u64 = 3600;
const SECS_IN_DAY: u64 = 3600 * 24;

pub trait DurationExt {
    fn from_minutes(minutes: u64) -> Duration;
    fn from_hours(hours: u64) -> Duration;
    fn from_days(days: u64) -> Duration;

    fn add_nanos(self, nanos: u64) -> Duration;
    fn add_micros(self, micros: u64) -> Duration;
    fn add_millis(self, millis: u64) -> Duration;
    fn add_secs(self, seconds: u64) -> Duration;
    fn add_minutes(self, minutes: u64) -> Duration;
    fn add_hours(self, hours: u64) -> Duration;
    fn add_days(self, days: u64) -> Duration;

    fn as_minutes(&self) -> u64;
    fn as_hours(&self) -> u64;
    fn as_days(&self) -> u64;
}

impl DurationExt for Duration {
    fn from_minutes(minutes: u64) -> Self {
        let seconds = minutes * SECS_IN_MIN;
        Self::from_secs(seconds)
    }

    fn from_hours(hours: u64) -> Self {
        let seconds: u64 = hours * SECS_IN_HOUR;
        Self::from_secs(seconds)
    }

    fn from_days(days: u64) -> Self {
        let seconds = days * SECS_IN_DAY;
        Self::from_secs(seconds)
    }

    fn add_nanos(self, nanos: u64) -> Self {
        self + Self::from_nanos(nanos)
    }

    fn add_micros(self, micros: u64) -> Self {
        self + Self::from_micros(micros)
    }

    fn add_millis(self, millis: u64) -> Self {
        self + Self::from_millis(millis)
    }

    fn add_secs(self, seconds: u64) -> Self {
        self + Self::from_secs(seconds)
    }

    fn add_minutes(self, minutes: u64) -> Self {
        self + Self::from_minutes(minutes)
    }

    fn add_hours(self, hours: u64) -> Self {
        self + Self::from_hours(hours)
    }

    fn add_days(self, days: u64) -> Self {
        self + Self::from_days(days)
    }

    fn as_minutes(&self) -> u64 {
        self.as_secs() / SECS_IN_MIN
    }

    fn as_hours(&self) -> u64 {
        self.as_secs() / SECS_IN_HOUR
    }

    fn as_days(&self) -> u64 {
        self.as_secs() / SECS_IN_DAY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_methods() {
        // Check `from_minutes`
        let test_vector = vec![0, 1, u64::max_value() / SECS_IN_MIN];
        for minutes in test_vector {
            let seconds = minutes * SECS_IN_MIN;
            assert_eq!(
                Duration::from_minutes(minutes),
                Duration::from_secs(seconds)
            );
        }

        // Check `from_hours`
        let test_vector = vec![0, 1, u64::max_value() / SECS_IN_HOUR];
        for hours in test_vector {
            let seconds = hours * SECS_IN_HOUR;
            assert_eq!(Duration::from_hours(hours), Duration::from_secs(seconds));
        }

        // Check `from_days`
        let test_vector = vec![0, 1, u64::max_value() / SECS_IN_DAY];
        for days in test_vector {
            let seconds = days * SECS_IN_DAY;
            assert_eq!(Duration::from_days(days), Duration::from_secs(seconds));
        }
    }

    #[test]
    fn add_methods() {
        let duration = Duration::default()
            .add_nanos(1)
            .add_micros(1)
            .add_millis(1)
            .add_secs(1)
            .add_minutes(1)
            .add_hours(1)
            .add_days(1);

        let expected_duration = Duration::new(
            1 * SECS_IN_DAY + 1 * SECS_IN_HOUR + 1 * SECS_IN_MIN + 1,
            1 * 1_000_000 + 1 * 1_000 + 1,
        );

        assert_eq!(duration, expected_duration);
    }

    #[test]
    fn as_methods() {
        let test_vector = vec![0, SECS_IN_MIN, SECS_IN_HOUR, SECS_IN_DAY];

        for seconds in test_vector {
            for seconds in &[seconds, seconds + 1, seconds * 2, seconds * 2 + 1] {
                let duration = Duration::from_secs(*seconds);

                assert_eq!(duration.as_minutes(), duration.as_secs() / SECS_IN_MIN);
                assert_eq!(duration.as_hours(), duration.as_secs() / SECS_IN_HOUR);
                assert_eq!(duration.as_days(), duration.as_secs() / SECS_IN_DAY);
            }
        }
    }
}

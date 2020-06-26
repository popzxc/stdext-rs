use std::time::Duration;

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
}

impl DurationExt for Duration {
    fn from_minutes(minutes: u64) -> Self {
        let seconds = minutes * 60;
        Self::from_secs(seconds)
    }

    fn from_hours(hours: u64) -> Self {
        let seconds: u64 = hours * 3600;
        Self::from_secs(seconds)
    }

    fn from_days(days: u64) -> Self {
        let hours = days * 24;
        Self::from_hours(hours)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_minutes() {
        const SECS_IN_MIN: u64 = 60;

        let test_vector = vec![0, 1, u64::max_value() / SECS_IN_MIN];

        for minutes in test_vector {
            let seconds = minutes * SECS_IN_MIN;
            assert_eq!(
                Duration::from_minutes(minutes),
                Duration::from_secs(seconds)
            );
        }
    }

    #[test]
    fn from_hours() {
        const SECS_IN_HOUR: u64 = 3600;

        let test_vector = vec![0, 1, u64::max_value() / SECS_IN_HOUR];

        for hours in test_vector {
            let seconds = hours * SECS_IN_HOUR;
            assert_eq!(Duration::from_hours(hours), Duration::from_secs(seconds));
        }
    }

    #[test]
    fn from_days() {
        const SECS_IN_DAY: u64 = 3600 * 24;

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
            1 * (3600 * 24) + 1 * 3600 + 1 * 60 + 1,
            1 * 1_000_000 + 1 * 1_000 + 1,
        );

        assert_eq!(duration, expected_duration);
    }
}

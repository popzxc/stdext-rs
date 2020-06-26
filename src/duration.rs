use std::time::Duration;

pub trait DurationExt {
    fn from_mins(minutes: u64) -> Duration;
    fn from_hours(hours: u64) -> Duration;
    fn from_days(days: u64) -> Duration;
}

impl DurationExt for Duration {
    fn from_mins(minutes: u64) -> Self {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_mins() {
        const SECS_IN_MIN: u64 = 60;

        let test_vector = vec![0, 1, u64::max_value() / SECS_IN_MIN];

        for minutes in test_vector {
            let seconds = minutes * SECS_IN_MIN;
            assert_eq!(Duration::from_mins(minutes), Duration::from_secs(seconds));
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
}
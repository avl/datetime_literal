pub use chrono;

/// Creates chrono::DateTime values.
///
/// The syntax is inspired by ISO 8601, but because of how rust macros work it cannot
/// support the exact same format.
///
/// Examples:
///
/// ```rust
///     use datetime_literal::datetime;
///
///     assert_eq!(
///         datetime!(2024-01-02 13:14:15),
///         "2024-01-02T13:14:15".parse().unwrap()
///     );
///     assert_eq!(
///         datetime!(2024-01-02 13:14:15 Z),
///         "2024-01-02T13:14:15Z".parse::<chrono::DateTime<chrono::Utc>>().unwrap()
///     );
///
///     assert_eq!(
///         datetime!(2024-01-02 T 13:14:15),
///         "2024-01-02T13:14:15".parse().unwrap()
///     );
///     assert_eq!(
///         datetime!(2024-01-02 T 13:14:15 Z),
///         "2024-01-02T13:14:15Z".parse::<chrono::DateTime<chrono::Utc>>().unwrap()
///     );
///
///     assert_eq!(
///         datetime!(2024-01-02),
///         "2024-01-02T00:00:00".parse().unwrap()
///     );
///     assert_eq!(
///         datetime!(2024-01-02 Z),
///         "2024-01-02T00:00:00Z".parse::<chrono::DateTime<chrono::Utc>>().unwrap()
///     );
///
/// ```
/// Note! Because of how rust macros work, you must leave space before/after 'T', and before 'Z'.
///
#[macro_export]
macro_rules! datetime {
    ( $year:literal-$month:literal-$day:literal Z)  => {

        const {
            #[allow(clippy::zero_prefixed_literal)]
            $crate::chrono::DateTime::<$crate::chrono::Utc>::from_naive_utc_and_offset(
            datetime!($year - $month - $day),
            $crate::chrono::Utc
        ) }
    };
    ( $year:literal-$month:literal-$day:literal)  => {

        const {
            #[allow(clippy::zero_prefixed_literal)]
            $crate::chrono::NaiveDateTime::new(

            $crate::chrono::NaiveDate::from_ymd_opt($year, $month, $day).expect("date values must be in valid range"),
            $crate::chrono::NaiveTime::from_hms_opt(0,0,0).unwrap()
        ) }
    };
    ( $year:literal-$month:literal-$day:literal $(T)? $hour:literal:$min:literal:$second:literal Z)  => {

        const {
            #[allow(clippy::zero_prefixed_literal)]
            $crate::chrono::DateTime::<$crate::chrono::Utc>::from_naive_utc_and_offset(

            datetime!($year - $month - $day $hour:$min:$second),
            $crate::chrono::Utc
        ) }
    };
    ( $year:literal-$month:literal-$day:literal $(T)? $hour:literal:$min:literal:$second:literal)  => {

        const {
            #[allow(clippy::zero_prefixed_literal)]
            $crate::chrono::NaiveDateTime::new(
            $crate::chrono::NaiveDate::from_ymd_opt($year, $month, $day).expect("date values must be in valid range"),
            $crate::chrono::NaiveTime::from_hms_opt($hour, $min, $second).expect("time values must be in valid range")
        ) }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(datetime!(1970-01-01 0:00:00 Z).timestamp(), 0);
    }
}

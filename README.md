# Datetime Literal

This is an extremely simple crate that provides a macro for easily creating instances of chrono's DateTime:

Examples:

```rust
assert_eq!(
    datetime!(2024-01-02 13:14:15),
    "2024-01-02T13:14:15".parse().unwrap()
);
assert_eq!(
    datetime!(2024-01-02 13:14:15 Z),
    "2024-01-02T13:14:15Z".parse::<chrono::DateTime<chrono::Utc>>().unwrap()
);

assert_eq!(
    datetime!(2024-01-02 T 13:14:15),
    "2024-01-02T13:14:15".parse().unwrap()
);
assert_eq!(
    datetime!(2024-01-02 T 13:14:15 Z),
    "2024-01-02T13:14:15Z".parse::<chrono::DateTime<chrono::Utc>>().unwrap()
);

assert_eq!(
    datetime!(2024-01-02),
    "2024-01-02T00:00:00".parse().unwrap()
);
assert_eq!(
    datetime!(2024-01-02 Z),
    "2024-01-02T00:00:00Z".parse::<chrono::DateTime<chrono::Utc>>().unwrap()
);

```

Note! Because of how rust macros work, you must leave space before/after 'T', and before 'Z'.

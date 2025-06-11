use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let one_gig: i64 = 1_000_000_000;
    let one_gigasec = Duration::new(one_gig, 0);
    start + one_gigasec
}

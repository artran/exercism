use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let one_gig: i64 = 1_000_000_000;
    let one_gigasec = Duration::seconds(one_gig);
    start + one_gigasec
}

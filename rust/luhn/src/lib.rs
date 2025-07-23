use regex_lite::Regex;


/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let valid_luhn_re = Regex::new(r"^\d\d+$").unwrap();

    let cleaned: String = code.chars()
        .filter(|&c| !c.is_whitespace())
        .rev()
        .collect();

    if !valid_luhn_re.is_match(&cleaned) {
        return false;
    }

    let luhn: u32 = cleaned.chars()
        .enumerate()
        .map(|(i, c)| {
            let x = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                x
            } else {
                double_digit_with_restriction(x)
            }
        })
        .sum();

    luhn % 10 == 0
}

fn double_digit_with_restriction(x: u32) -> u32 {
    let doubled = x * 2;
    if doubled > 9 {
        return doubled - 9
    }

    doubled
}

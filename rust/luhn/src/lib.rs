/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // Process characters from right to left, skipping whitespace.
    // 'try_fold' allows us to calculate the sum while also handling invalid characters.
    let luhn_result = code
        .chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        // Use a two-tuple to track the luhn sum and the character count
        .try_fold((0, 0), |(sum, count), (index, c)| {
            c.to_digit(10)
                .map(|digit| {
                    let term = if index % 2 == 1 {
                        double_digit_with_restriction(digit)
                    } else {
                        digit
                    };
                    (sum + term, count + 1)
                })
        });

    // The final result is valid if:
    // 1. No invalid characters were found (luhn_result is Some).
    // 2. The length is greater than 1.
    // 3. The sum is divisible by 10.
    match luhn_result {
        Some((sum, count)) if count > 1 => sum % 10 == 0,
        _ => false,
    }
}

fn double_digit_with_restriction(x: u32) -> u32 {
    let doubled = x * 2;
    if doubled > 9 {
        doubled - 9
    } else {
        doubled
    }
}

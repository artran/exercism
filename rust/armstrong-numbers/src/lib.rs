pub fn is_armstrong_number(num: u32) -> bool {
    let power = match num {
        0..9 => 1, // Zero would panic in ilog10 and small optimisation including 1..9
        _ => num.ilog10() + 1,
    };

    num.to_string()
        .chars()
        .fold(0, |acc, x| acc + x.to_digit(10).unwrap().pow(power))
        == num
}

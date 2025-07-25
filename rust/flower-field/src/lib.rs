const OFFSETS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.bytes()
                .enumerate()
                .map(|(c, letter)| match letter {
                    b'*' => '*',
                    _ => count_neighbours(garden, r, c),
                })
                .collect()
        })
        .collect()
}

fn count_neighbours(garden: &[&str], r: usize, c: usize) -> char {
    let height = garden.len();
    let width = garden[0].len();

    let count = OFFSETS
        .iter()
        .map(|&(dx, dy)| (c.wrapping_add_signed(dx), r.wrapping_add_signed(dy)))
        .filter(|&(x, y)| x < width && y < height && garden[y].as_bytes()[x] == b'*')
        .count();

    match count {
        0 => ' ',
        _ => char::from_digit(count as u32, 10).unwrap(),
    }
}

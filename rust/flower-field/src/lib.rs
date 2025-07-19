pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.bytes()
                .enumerate()
                .map(|(c, letter)| match letter {
                    b'*' => '*',
                    _ => count_neighbours(r, c),
                })
                .collect()
        })
        .collect()
}

fn count_neighbours(r: usize, c: usize) -> char {
    ' '
}

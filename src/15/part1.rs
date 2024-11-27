pub fn solve(input: &str) -> usize {
    input.trim_end().split(',').map(hash).sum()
}

// Determine the ASCII code for the current character of the string.
// Increase the current value by the ASCII code you just determined.
// Set the current value to itself multiplied by 17.
// Set the current value to the remainder of dividing itself by 256.
fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

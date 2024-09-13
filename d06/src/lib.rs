use std::collections::VecDeque;

/// Returns the index of the last element in the window,
/// if such a window exists
fn get_first_buffer_all_unique(string: &str, buffer_size: usize) -> Option<usize> {
    let mut buf: VecDeque<char> = VecDeque::with_capacity(buffer_size);
    let mut chars_to_skip: usize = 0;
    for (idx, letter) in string.char_indices() {
        while buf.contains(&letter) {
            buf.pop_front();
            chars_to_skip += 1;
        }
        buf.push_back(letter);
        chars_to_skip = chars_to_skip.saturating_sub(1);
        if buf.len() == buffer_size {
            return Some(idx + 1);
        }
    }
    None
}

pub fn p1(buffer: &str) -> Option<usize> {
    get_first_buffer_all_unique(buffer, 4)
}
pub fn p2(buffer: &str) -> Option<usize> {
    get_first_buffer_all_unique(buffer, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EX1: &str = include_str!("../inputs/examples/1");
    const EX2: &str = include_str!("../inputs/examples/2");
    const EX3: &str = include_str!("../inputs/examples/3");
    const EX4: &str = include_str!("../inputs/examples/4");
    const EX5: &str = include_str!("../inputs/examples/5");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EX1 => 7)]
    #[test_case(EX2 => 5)]
    #[test_case(EX3 => 6)]
    #[test_case(EX4 => 10)]
    #[test_case(EX5 => 11)]
    #[test_case(REAL => 1142)]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }

    #[test_case(EX1 => 19)]
    #[test_case(EX2 => 23)]
    #[test_case(EX3 => 23)]
    #[test_case(EX4 => 29)]
    #[test_case(EX5 => 26)]
    #[test_case(REAL => 2803)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}

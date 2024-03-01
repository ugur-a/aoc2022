use std::collections::VecDeque;

/// Returns the index of the last element in the window,
/// if such a window exists
fn get_first_buffer_all_unique(string: &str, buffer_size: usize) -> Option<usize> {
    let mut buf: VecDeque<char> = VecDeque::with_capacity(buffer_size);
    let mut string = string.char_indices();
    let mut chars_to_skip = 0usize;
    while let Some((idx, letter)) = string.next() {
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

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb" => Some(7))]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz" => Some(5))]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg" => Some(6))]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => Some(10))]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => Some(11))]
    fn test_p1(inp: &str) -> Option<usize> {
        p1(inp)
    }
}

pub fn p2(_file: &str) -> anyhow::Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn example() {
        let inp = read_to_string("inputs/example.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 56 * 62);
    }

    #[test]
    #[ignore]
    fn real() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn example() {
        let inp = read_to_string("inputs/example.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 33);
    }

    #[test]
    #[ignore]
    fn real() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 0);
    }
}

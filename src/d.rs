use std::str::FromStr;

pub fn p1(file: &str) -> anyhow::Result<u32> {
    todo!()
}
pub fn p2(_file: &str) -> anyhow::Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let inp = include_str!("../inputs/d20/test.txt");
        assert_eq!(p1(inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = include_str!("../inputs/d20/real.txt");
        assert_eq!(p1(inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = include_str!("../inputs/d20/test.txt");
        assert_eq!(p2(inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = include_str!("../inputs/d20/real.txt");
        assert_eq!(p2(inp).unwrap(), 0);
    }
}

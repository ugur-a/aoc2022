pub fn p1(file: &str) -> () {
    todo!()
}
pub fn p2(file: &str) -> () {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d10/test.txt").unwrap();
        assert_eq!(p1(&inp), 21);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/d10/real.txt").unwrap();
        assert_eq!(p1(&inp), 1708);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/d10/test.txt").unwrap();
        assert_eq!(p2(&inp), 8);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d10/real.txt").unwrap();
        assert_eq!(p2(&inp), 504_000);
    }
}

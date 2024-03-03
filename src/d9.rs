pub fn p1(file: &str) -> usize {
    todo!()
}
pub fn p2(file: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn move_2_rope() {
        let mut rope = Rope::with_length(2);
        rope.r#move(Direction::Up);
        assert_eq!(rope, vec![Point2D { x: 0, y: 1 }, Point2D { x: 0, y: 0 }]);
    }
    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d9/test.txt").unwrap();
        assert_eq!(p1(&inp), 13);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/d9/real.txt").unwrap();
        assert_eq!(p1(&inp), 5960);
    }
}

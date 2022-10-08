pub mod rs0 {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod test_rs0 {
    use super::rs0::*;

    #[test]
    fn test_add() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(0, 1), 1);
        assert_eq!(add(20, 0), 20);
        assert_eq!(add(-20, 20), 0);
        assert_eq!(add(-2, 20), 18);
    }
}

pub mod rs0 {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn add_3(a: i32, b: i32, c: i32) -> i32 {
        a + b + c
    }

    pub fn greater_than_5(x: i32) -> bool {
        x > 5
    }

    pub fn largest<T: Ord>(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
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

    #[test]
    fn test_add_3() {
        assert_eq!(add_3(0, 0, 0), 0);
        assert_eq!(add_3(0, 1, 20), 21);
        assert_eq!(add_3(20, 0, -100), -80);
        assert_eq!(add_3(-20, 0, 20), 0);
        assert_eq!(add_3(-2, 20, 200), 218);
    }

    #[test]
    fn test_greater_than_5() {
        assert!(greater_than_5(6));
        assert!(greater_than_5(20));
        assert!(!greater_than_5(5));
        assert!(!greater_than_5(-75));
        assert!(!greater_than_5(0));
    }

    #[test]
    fn test_largest() {
        assert_eq!(largest(0, 0), 0);
        assert_eq!(largest(0, 1), 1);
        assert_eq!(largest(0, -1), 0);
        assert_eq!(largest(2, 10), 10);
        assert_eq!(largest(-10, 2), 2);

        assert_eq!(largest("abc", "bcd"), "bcd");
        assert_eq!(largest('b', 'a'), 'b');
        assert_eq!(largest(true, false), true);
        assert_eq!(
            largest(String::from("a"), String::from("z")),
            String::from("z")
        );
        assert_eq!(largest(Some(-1), None), Some(-1));
    }
}

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

    pub fn largest_3<T: Ord>(a: T, b: T, c: T) -> T {
        if a > b && a > c {
            a
        } else if b > c {
            b
        } else {
            c
        }
    }

    pub fn is_first_larger<T: Ord>(first: T, second: T) -> bool {
        first > second
    }

    pub fn is_sum_greater_than_10(a: i32, b: i32) -> bool {
        a + b > 10
    }

    pub fn lazy_add() -> fn(i32, i32) -> i32 {
        |a, b| a + b
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

    #[test]
    fn test_largest_3() {
        assert_eq!(largest_3(0, 0, 0), 0);
        assert_eq!(largest_3(-1, 0, 1), 1);
        assert_eq!(largest_3(-1, 0, -1), 0);
        assert_eq!(largest_3(0, 2, 10), 10);
        assert_eq!(largest_3(2, -10, -2), 2);

        assert_eq!(largest_3("aaa", "abc", "bcd"), "bcd");
        assert_eq!(largest_3('a', 'b', 'a'), 'b');
        assert_eq!(largest_3(false, true, false), true);
        assert_eq!(
            largest_3(String::from("x"), String::from("a"), String::from("z")),
            String::from("z")
        );
        assert_eq!(largest_3(Some(-15), Some(-1), None), Some(-1));
    }

    #[test]
    fn test_is_first_larger() {
        assert!(!is_first_larger(1, 2));
        assert!(is_first_larger(2, 1));
        assert!(!is_first_larger("abc", "xyz"));
        assert!(is_first_larger("xyz", "abc"));
        assert!(!is_first_larger("10", "2"));
        assert!(is_first_larger("2", "10"));
    }

    #[test]
    fn test_is_sum_greater_than_10() {
        assert!(!is_sum_greater_than_10(10, 0));
        assert!(is_sum_greater_than_10(10, 1));
        assert!(!is_sum_greater_than_10(-10, -1));
    }

    #[test]
    fn test_lazy_add() {
        assert_eq!(lazy_add()(1, 10), 11);
        assert_eq!(lazy_add()(1, -10), -9);
        assert_eq!(lazy_add()(0, -1), -1);
    }
}

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

    pub fn largest<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
    }

    pub fn largest_3<T: PartialOrd>(a: T, b: T, c: T) -> T {
        if a > b && a > c {
            a
        } else if b > c {
            b
        } else {
            c
        }
    }

    pub fn is_first_larger<T: PartialOrd>(first: T, second: T) -> bool {
        first > second
    }

    pub fn is_sum_greater_than_10(a: i32, b: i32) -> bool {
        a + b > 10
    }

    pub fn lazy_add() -> fn(i32, i32) -> i32 {
        |a, b| a + b
    }

    pub fn lazy_add_3(a: i32, b: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |c| a + b + c)
    }

    pub fn function_add(a: fn() -> i32, b: fn() -> i32) -> i32 {
        a() + b()
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

    #[test]
    fn test_lazy_add_3() {
        assert_eq!(lazy_add_3(1, 10)(100), 111);
        assert_eq!(lazy_add_3(-1, 2)(-10), -9);
        assert_eq!(lazy_add_3(0, -1)(2), 1);
    }

    #[test]
    fn test_function_add() {
        assert_eq!(function_add(|| 1, || 2), 3);
        assert_eq!(function_add(|| 100, || 10), 110);
        assert_eq!(function_add(|| 100, || -1000), -900);
    }
}

pub mod rs1 {
    pub fn make_incrementer(mut count: i32) -> Box<dyn FnMut() -> i32> {
        Box::new(move || {
            count += 1;
            count
        })
    }

    pub fn call_max_times<T: ?Sized>(
        mut max_times: i32,
        f: fn() -> &'static T,
    ) -> Box<dyn FnMut() -> Option<&'static T>> {
        Box::new(move || {
            if max_times <= 0 {
                return None;
            }
            max_times -= 1;
            Some(f())
        })
    }

    pub fn repeat_str(s: &str, times: i32) -> String {
        let mut repeated = String::new();
        for _ in 0..times {
            repeated.push_str(s);
        }
        repeated
    }

    pub fn call_while_true(f: fn(i32) -> bool) -> i32 {
        let mut i = 0;
        while f(i) {
            i += 1;
        }
        i
    }

    pub fn call_times(f: fn(), mut times: i32) {
        while times > 0 {
            f();
            times -= 1;
        }
    }

    pub fn is_prime(x: i32) -> bool {
        x >= 2 && (2..x).all(|divisor| x % divisor != 0)
    }

    pub trait Container<T: PartialEq> {
        fn has(&self, item: T) -> bool;
        fn count(&self, item: T) -> usize;
    }

    impl Container<char> for &str {
        fn has(&self, item: char) -> bool {
            self.contains(item)
        }

        fn count(&self, item: char) -> usize {
            self.chars().filter(|ch| *ch == item).count()
        }
    }

    impl Container<char> for String {
        fn has(&self, item: char) -> bool {
            self.contains(item)
        }

        fn count(&self, item: char) -> usize {
            self.chars().filter(|ch| *ch == item).count()
        }
    }

    impl<T: PartialEq> Container<T> for Vec<T> {
        fn has(&self, item: T) -> bool {
            self.iter().any(|x| *x == item)
        }

        fn count(&self, item: T) -> usize {
            self.iter().filter(|x| **x == item).count()
        }
    }

    pub fn contains<T: PartialEq>(s: impl Container<T>, ch: T) -> bool {
        s.has(ch)
    }

    pub fn count<T: PartialEq>(container: impl Container<T>, item: T) -> usize {
        container.count(item)
    }

    pub fn map_str(s: &str, f: fn(char) -> String) -> String {
        let mut result = String::new();
        for ch in s.chars() {
            result += &f(ch);
        }
        result
    }

    pub fn make_letter_looper(s: &str) -> Box<dyn FnMut() -> Option<char> + '_> {
        let mut i = 0;
        Box::new(move || {
            let result = s.chars().nth(i);
            i = (i + 1) % s.len().max(1);
            result
        })
    }

    pub async fn wait_then_call(wait_ms: u64, f: fn()) {
        use tokio::time::{sleep, Duration};
        sleep(Duration::from_millis(wait_ms)).await;
        f();
    }
}

#[cfg(test)]
mod test_rs1 {
    use super::rs1::*;

    #[test]
    fn test_make_incrementer() {
        let mut incrementer = make_incrementer(0);
        assert_eq!(incrementer(), 1);
        assert_eq!(incrementer(), 2);
        assert_eq!(incrementer(), 3);

        let mut incrementer = make_incrementer(-5);
        assert_eq!(incrementer(), -4);
        assert_eq!(incrementer(), -3);
        assert_eq!(incrementer(), -2);
    }

    #[test]
    fn test_call_max_times() {
        let mut caller = call_max_times(3, || "hello");
        assert_eq!(caller(), Some("hello"));
        assert_eq!(caller(), Some("hello"));
        assert_eq!(caller(), Some("hello"));
        assert_eq!(caller(), None);

        let mut caller = call_max_times(2, || &123);
        assert_eq!(caller(), Some(&123));
        assert_eq!(caller(), Some(&123));
        assert_eq!(caller(), None);
    }

    #[test]
    fn test_repeat_str() {
        assert_eq!(repeat_str("a", 5), "aaaaa");
        assert_eq!(repeat_str("ab", 3), "ababab");
        assert_eq!(repeat_str("abc", 0), "");
        assert_eq!(repeat_str("abc", -1), "");
        assert_eq!(repeat_str("abc", 1), "abc");
        assert_eq!(repeat_str("abc", 2), "abcabc");
    }

    #[test]
    fn test_call_while_true() {
        assert_eq!(call_while_true(|x| x < 6), 6);
        assert_eq!(call_while_true(|_| false), 0);
    }

    #[test]
    fn test_call_times() {
        call_times(|| println!("hey"), -5);
        call_times(|| println!("hi"), 0);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(-5), false);
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(11), true);
    }

    #[test]
    fn test_contains() {
        assert_eq!(contains("abc", 'a'), true);
        assert_eq!(contains("abc", 'b'), true);
        assert_eq!(contains("abc", 'c'), true);
        assert_eq!(contains("abc", 'd'), false);

        assert_eq!(contains(String::from("abc"), 'a'), true);
        assert_eq!(contains(String::from("abc"), 'b'), true);
        assert_eq!(contains(String::from("abc"), 'c'), true);
        assert_eq!(contains(String::from("abc"), 'd'), false);

        assert_eq!(contains(vec!['a', 'b', 'c'], 'a'), true);
        assert_eq!(contains(vec!['a', 'b', 'c'], 'b'), true);
        assert_eq!(contains(vec!['a', 'b', 'c'], 'c'), true);
        assert_eq!(contains(vec!['a', 'b', 'c'], 'd'), false);

        assert_eq!(contains(vec![1, 2, 3], 1), true);
        assert_eq!(contains(vec![1, 2, 3], 2), true);
        assert_eq!(contains(vec![1, 2, 3], 3), true);
        assert_eq!(contains(vec![1, 2, 3], 4), false);
    }

    #[test]
    fn test_count() {
        assert_eq!(count("abc", 'a'), 1);
        assert_eq!(count("aba", 'a'), 2);
        assert_eq!(count("aba", 'b'), 1);
        assert_eq!(count("aba", 'd'), 0);

        assert_eq!(count(String::from("abc"), 'a'), 1);
        assert_eq!(count(String::from("aba"), 'a'), 2);
        assert_eq!(count(String::from("aba"), 'b'), 1);
        assert_eq!(count(String::from("aba"), 'd'), 0);

        assert_eq!(count(vec!['a', 'b', 'a'], 'a'), 2);
        assert_eq!(count(vec!['a', 'b', 'a'], 'b'), 1);
        assert_eq!(count(vec!['a', 'b', 'a'], 'd'), 0);

        assert_eq!(count(vec![1, 2, 3], 1), 1);
        assert_eq!(count(vec![1, 2, 3], 2), 1);
        assert_eq!(count(vec![1, 2, 3], 3), 1);
        assert_eq!(count(vec![1, 2, 3], 4), 0);
        assert_eq!(count(vec![1, 2, 1], 1), 2);
    }

    #[test]
    fn test_map_str() {
        assert_eq!(map_str("hello", |_| String::from("9")), "99999");
        assert_eq!(
            map_str("hello", |x| {
                let mut result = String::new();
                result.push(x);
                result += "!";
                result
            }),
            "h!e!l!l!o!"
        );
        assert_eq!(
            map_str("hi", |x| {
                let mut result = String::new();
                result.push(x);
                result += "123";
                result
            }),
            "h123i123"
        );
    }

    #[test]
    fn test_make_letter_looper() {
        let mut letter_looper = make_letter_looper("");
        assert_eq!(letter_looper(), None);
        assert_eq!(letter_looper(), None);
        assert_eq!(letter_looper(), None);

        let mut letter_looper = make_letter_looper("abc");
        assert_eq!(letter_looper(), Some('a'));
        assert_eq!(letter_looper(), Some('b'));
        assert_eq!(letter_looper(), Some('c'));
        assert_eq!(letter_looper(), Some('a'));
        assert_eq!(letter_looper(), Some('b'));
        assert_eq!(letter_looper(), Some('c'));
        assert_eq!(letter_looper(), Some('a'));
    }

    #[tokio::test]
    async fn test_wait_then_call() {
        tokio::join!(
            wait_then_call(2, || println!("hi after 2ms")),
            wait_then_call(1, || println!("hi after 1ms"))
        );
    }
}

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

    pub async fn wait_twice_call(wait_ms: u64, f: fn()) {
        use tokio::time::{sleep, Duration};
        sleep(Duration::from_millis(wait_ms)).await;
        f();
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

    #[tokio::test]
    async fn test_wait_twice_call() {
        wait_twice_call(3, || println!("hi")).await;
    }
}

pub mod rs2 {
    pub fn make_vec(size: i32) -> Vec<i32> {
        fn iter(size: i32, i: i32, mut result: Vec<i32>) -> Vec<i32> {
            if i >= size {
                return result;
            }
            result.push(i);
            iter(size, i + 1, result)
        }
        iter(size, 0, vec![])
    }

    pub fn make_vec_while_true(f: fn(i32) -> bool) -> Vec<i32> {
        fn iter(f: fn(i32) -> bool, i: i32, mut result: Vec<i32>) -> Vec<i32> {
            if !f(i) {
                return result;
            }
            result.push(i);
            iter(f, i + 1, result)
        }
        iter(f, 0, vec![])
    }

    pub fn make_matrix(rows: i32, cols: i32) -> Vec<Vec<i32>> {
        fn make_row(cols: i32, mut row: Vec<i32>) -> Vec<i32> {
            if row.len() as i32 >= cols {
                return row;
            }
            row.push(0);
            make_row(cols, row)
        }
        fn make_rows(rows: i32, cols: i32, mut result: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            if result.len() as i32 >= rows {
                return result;
            }
            result.push(make_row(cols, vec![]));
            make_rows(rows, cols, result)
        }
        make_rows(rows, cols, vec![])
    }

    pub fn make_map_looper<'a, T, U: 'a>(
        values: &'a Vec<T>,
        f: fn(&'a T) -> U,
    ) -> Box<dyn FnMut() -> Option<U> + 'a> {
        let mut i = 0;
        Box::new(move || {
            let result = values.get(i).map(f);
            i = (i + 1) % values.len().max(1);
            result
        })
    }

    pub async fn wait_then_call_functions(functions: &Vec<fn()>, wait_ms: u64) {
        use tokio::time::{sleep, Duration};
        sleep(Duration::from_millis(wait_ms)).await;
        functions.iter().for_each(|f| f());
    }

    pub async fn wait_before_each(functions: &Vec<fn()>, wait_ms: u64) {
        use tokio::time::{sleep, Duration};
        for f in functions {
            sleep(Duration::from_millis(wait_ms)).await;
            f();
        }
    }

    pub fn for_each<T>(values: &Vec<T>, f: fn(&T, usize, &Vec<T>)) {
        fn iter<T>(values: &Vec<T>, f: fn(&T, usize, &Vec<T>), i: usize) {
            if i >= values.len() {
                return;
            }
            f(&values[i], i, values);
            iter(values, f, i + 1)
        }
        iter(values, f, 0)
    }

    pub fn map<T, U>(values: &Vec<T>, f: fn(&T, usize, &Vec<T>) -> U) -> Vec<U> {
        fn iter<T, U>(
            values: &Vec<T>,
            f: fn(&T, usize, &Vec<T>) -> U,
            i: usize,
            mut mapped: Vec<U>,
        ) -> Vec<U> {
            if i >= values.len() {
                return mapped;
            }
            mapped.push(f(&values[i], i, values));
            iter(values, f, i + 1, mapped)
        }
        iter(values, f, 0, vec![])
    }

    pub fn reduce<T, U>(values: &Vec<T>, f: fn(U, &T, usize, &Vec<T>) -> U, init: U) -> U {
        fn iter<T, U>(
            values: &Vec<T>,
            f: fn(U, &T, usize, &Vec<T>) -> U,
            i: usize,
            result: U,
        ) -> U {
            if i >= values.len() {
                return result;
            }
            iter(values, f, i + 1, f(result, &values[i], i, values))
        }
        iter(values, f, 0, init)
    }

    pub fn filter<T>(values: &Vec<T>, f: fn(&T, usize, &Vec<T>) -> bool) -> Vec<&T> {
        fn iter<'a, T>(
            values: &'a Vec<T>,
            f: fn(&T, usize, &Vec<T>) -> bool,
            i: usize,
            mut filtered: Vec<&'a T>,
        ) -> Vec<&'a T> {
            if i >= values.len() {
                return filtered;
            }
            if f(&values[i], i, values) {
                filtered.push(&values[i]);
            }
            iter(values, f, i + 1, filtered)
        }
        iter(values, f, 0, vec![])
    }

    pub fn find<T>(values: &Vec<T>, f: fn(&T, usize, &Vec<T>) -> bool) -> Option<&T> {
        fn iter<T>(values: &Vec<T>, f: fn(&T, usize, &Vec<T>) -> bool, i: usize) -> Option<&T> {
            if i >= values.len() {
                return None;
            }
            if f(&values[i], i, values) {
                return Some(&values[i]);
            }
            iter(values, f, i + 1)
        }
        iter(values, f, 0)
    }
}

#[cfg(test)]
mod test_rs2 {
    use crate::rs2::*;

    #[test]
    fn test_make_vec() {
        assert_eq!(make_vec(0), vec![]);
        assert_eq!(make_vec(1), vec![0]);
        assert_eq!(make_vec(2), vec![0, 1]);
        assert_eq!(make_vec(3), vec![0, 1, 2]);
        assert_eq!(make_vec(4), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_make_vec_while_true() {
        assert_eq!(make_vec_while_true(|x| x < 0), vec![]);
        assert_eq!(make_vec_while_true(|x| x < 1), vec![0]);
        assert_eq!(make_vec_while_true(|x| x < 2), vec![0, 1]);
        assert_eq!(make_vec_while_true(|x| x < 3), vec![0, 1, 2]);
        assert_eq!(make_vec_while_true(|x| x < 4), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_make_matrix() {
        assert_eq!(make_matrix(5, 2), [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0]]);
        assert_eq!(make_matrix(2, 5), [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0]]);
        assert_eq!(make_matrix(-1, -1), vec![] as Vec<Vec<i32>>);
        assert_eq!(make_matrix(0, 3), vec![] as Vec<Vec<i32>>);
        assert_eq!(
            make_matrix(3, 0),
            vec![vec![], vec![], vec![]] as Vec<Vec<i32>>
        );
    }

    #[test]
    fn test_make_loop_mapper() {
        let empty = vec![];
        let mut loop_mapper = make_map_looper(&empty, |x| x + 1);
        assert_eq!(loop_mapper(), None);

        let nums = vec![5, 2, 1, 3];
        let mut loop_mapper = make_map_looper(&nums, |x| x + 1);
        assert_eq!(loop_mapper(), Some(6));
        assert_eq!(loop_mapper(), Some(3));
        assert_eq!(loop_mapper(), Some(2));
        assert_eq!(loop_mapper(), Some(4));
        assert_eq!(loop_mapper(), Some(6));

        let strs = vec!["hello", "what", "a", "day"];
        let mut loop_mapper = make_map_looper(&strs, |x| if x.len() < 2 { "" } else { x });
        assert_eq!(loop_mapper(), Some("hello"));
        assert_eq!(loop_mapper(), Some("what"));
        assert_eq!(loop_mapper(), Some(""));
        assert_eq!(loop_mapper(), Some("day"));
        assert_eq!(loop_mapper(), Some("hello"));
    }

    #[tokio::test]
    async fn test_wait_then_call_functions() {
        wait_then_call_functions(&vec![|| println!("hi!"), || println!("hi!!")], 7).await;
    }

    #[tokio::test]
    async fn test_wait_before_each() {
        wait_before_each(&vec![|| println!("hey!"), || println!("hey!!")], 8).await;
    }

    #[test]
    fn test_for_each() {
        for_each(&vec![5, 8, 7], |x, i, values| {
            println!("for_each {x} {i} {values:?}");
        });
    }

    #[test]
    fn test_map() {
        assert_eq!(
            map(&(0..10).collect(), |x, _, _| x * 2),
            [0, 2, 4, 6, 8, 10, 12, 14, 16, 18]
        );
        assert_eq!(
            map(&(0..5).collect(), |x, i, _| x * 2 + i * 10),
            [0, 12, 24, 36, 48]
        );
    }

    #[test]
    fn test_reduce() {
        assert_eq!(reduce(&vec![1, 10, 100], |a, b, _, _| a + b, 0), 111);
    }

    #[test]
    fn test_filter() {
        assert_eq!(
            filter(&vec![1, -1, 2, -2, -3, 3], |&x, i, nums| x > 0
                && i > 0
                && i < nums.len() - 1),
            vec![&2]
        );
    }

    #[test]
    fn test_find() {
        assert_eq!(
            find(&vec![1, -1, 2, -2, -3, 3], |&x, i, nums| x > 0
                && i > 0
                && i < nums.len() - 1),
            Some(&2)
        );
    }
}

pub mod rs3 {
    use std::collections::HashMap;
    use std::hash::Hash;

    pub fn pick_key_values<'a, K: Hash + Eq, V>(
        keys: &'a Vec<K>,
        map: &'a HashMap<K, V>,
    ) -> Vec<&'a V> {
        fn iter<'a, K: Hash + Eq, V>(
            keys: &'a Vec<K>,
            map: &'a HashMap<K, V>,
            i: usize,
            mut result: Vec<&'a V>,
        ) -> Vec<&'a V> {
            if i >= keys.len() {
                return result;
            }
            if let Some(value) = map.get(&keys[i]) {
                result.push(value);
            }
            iter(keys, map, i + 1, result)
        }
        iter(keys, map, 0, vec![])
    }

    #[derive(Debug, PartialEq)]
    pub struct Coordinate {
        pub x: i32,
        pub y: i32,
    }

    pub fn make_coordinate_matrix(rows: i32, cols: i32) -> Vec<Vec<Coordinate>> {
        fn make_row(y: i32, cols: i32, x: i32, mut result: Vec<Coordinate>) -> Vec<Coordinate> {
            if x >= cols {
                return result;
            }
            result.push(Coordinate { x, y });
            make_row(y, cols, x + 1, result)
        }
        fn make_rows(
            rows: i32,
            cols: i32,
            y: i32,
            mut result: Vec<Vec<Coordinate>>,
        ) -> Vec<Vec<Coordinate>> {
            if y >= rows {
                return result;
            }
            result.push(make_row(y, cols, 0, vec![]));
            make_rows(rows, cols, y + 1, result)
        }
        make_rows(rows, cols, 0, vec![])
    }

    pub fn make_map_picker<K: Hash + Eq + Clone + 'static, V: Copy>(
        keys: Vec<K>,
    ) -> Box<dyn Fn(HashMap<K, V>) -> HashMap<K, V>> {
        Box::new(move |map| {
            fn iter<K: Hash + Eq + Clone, V: Copy>(
                keys: &Vec<K>,
                map: &HashMap<K, V>,
                i: usize,
                mut result: HashMap<K, V>,
            ) -> HashMap<K, V> {
                if i >= keys.len() {
                    return result;
                }
                if let Some(&value) = map.get(&keys[i]) {
                    result.insert(keys[i].clone(), value);
                }
                iter(keys, map, i + 1, result)
            }
            iter(&keys, &map, 0, HashMap::new())
        })
    }

    pub fn two_sum(nums: &Vec<i32>, target: i32) -> bool {
        use std::collections::HashSet;
        fn iter(nums: &Vec<i32>, target: i32, i: usize, mut seen_nums: HashSet<i32>) -> bool {
            if i >= nums.len() {
                return false;
            }
            if seen_nums.contains(&(target - nums[i])) {
                return true;
            }
            seen_nums.insert(nums[i]);
            iter(nums, target, i + 1, seen_nums)
        }
        iter(nums, target, 0, HashSet::new())
    }
}

#[cfg(test)]
mod test_rs3 {
    use super::rs3::*;
    use std::collections::HashMap;

    #[test]
    fn test_pick_key_values() {
        let keys = vec!["a", "c"];
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        assert_eq!(pick_key_values(&keys, &map), vec![&1, &3]);
    }

    #[test]
    fn test_make_coordinate_matrix() {
        assert_eq!(
            make_coordinate_matrix(3, 2),
            [
                [Coordinate { x: 0, y: 0 }, Coordinate { x: 1, y: 0 }],
                [Coordinate { x: 0, y: 1 }, Coordinate { x: 1, y: 1 }],
                [Coordinate { x: 0, y: 2 }, Coordinate { x: 1, y: 2 }]
            ]
        );
    }

    #[test]
    fn test_make_map_picker() {
        let map_picker = make_map_picker(vec!["a", "c"]);
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        let mut result = HashMap::new();
        result.insert("a", 1);
        result.insert("c", 3);
        assert_eq!(map_picker(map), result);

        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let mut result = HashMap::new();
        result.insert("a", 1);
        assert_eq!(map_picker(map), result);
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(&vec![1, 10, 100], 110), true);
        assert_eq!(two_sum(&vec![1, 10, 100], 1), false);
        assert_eq!(two_sum(&vec![1, 10, 100], 111), false);
        assert_eq!(two_sum(&vec![1, 10, 100], 11), true);
        assert_eq!(two_sum(&vec![1, 10, 100], 101), true);
    }
}

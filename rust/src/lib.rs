
// Decending Order
fn decending_order(n: u64) -> u64 {
    let mut list: Vec<char> = n.to_string().chars().collect();
    list.sort_by(|a, b| b.cmp(a));

    String::from_iter(list).parse().unwrap()
}

// Tribonacci
fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    let mut v: Vec<f64> = signature.to_vec();

    for i in 3..n {
        v.push(v[i-3] + v[i-2] + v[i-1]);
    }

    v[0..n].to_vec()
}

// Regex validate PIN code
use regex::Regex;

fn validate_pin(pin: &str) -> bool {
    let pin_regex = Regex::new(r"^\d{4}$|^\d{6}$").unwrap();
    pin_regex.is_match(pin)
}

// Sum of odd numbers
fn row_sum_odd_numbers(n: i64) -> i64 {
    n.pow(3) // Math stuff go brr
}

// Mumbling
fn accum(s: &str) -> String {
    let mut ans = String::new();
    let mut p_chars = s.char_indices().peekable();

    while p_chars.peek().is_some() {
        let i_char = p_chars.next().unwrap();
        ans.push(i_char.1.to_ascii_uppercase());

        for _ in 0..i_char.0 {
            ans.push(i_char.1.to_ascii_lowercase());
        }

        if p_chars.peek().is_some() {
            ans.push('-');
        }
    }

    ans
}

// Convert string to camel case
fn to_camel_case(text: &str) -> String {
    text.split(|ch| ch == '-' || ch == '_').enumerate().map(|(n, s)| {
        let mut chars = s.chars();
        match chars.next() {
            None => String::from(""),
            Some(mut c) => {
                if n != 0 { c = c.to_ascii_uppercase(); }
                String::from(c) + chars.as_str()
            }
        }
    }).collect::<String>()
}

// Symetric difference
fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|t| !b.contains(t)).collect::<Vec<T>>()
}

// Shortest Word
fn find_short(s: &str) -> u32 {
    let mut split: Vec<&str> = s.split(" ").collect();
    split.sort_by(|a,b| a.len().cmp(&b.len()));
    split.get(0).unwrap().len() as u32
}

// Highest Scoring Word
fn high(input: &str) -> &str {
    let mut pairs = input.split(" ").map(|s| {
        let chars_sum: u32 = s.chars()
            .map(|c| c.to_ascii_lowercase().to_digit(36).unwrap() - 9).sum(); // radix = 36 because 0-9 + 26 letters in the alphabet, subtract 9 for only letters
        (s, chars_sum)
    }).collect::<Vec<_>>();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));
    pairs.get(0).unwrap().0
}

// Human Readable Time
fn make_readable(seconds: u32) -> String {
    format!("{:0>2}:{:0>2}:{:0>2}", seconds / 3600, seconds / 60 % 60, seconds % 60)
}

// Persistent Bugger.
fn persistence(num: u64) -> u64 {
    let mut num = num;
    
    for i in 0.. {
        if num < 10 { return i }
        num = num.to_string().chars().map(|c| c.to_digit(10).unwrap() as u64).product();   
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_decending_order_test(n: u64, ans: u64) {
        let result = decending_order(n);
        assert_eq!(result, ans);
    }

    #[test]
    fn decending_order_tests() {
        do_decending_order_test(283, 832);
        do_decending_order_test(138937, 987331);
    }

    fn do_tribonacci_test(signature: &[f64; 3], n: usize, ans: Vec<f64>) {
        let result = tribonacci(signature, n);
        assert_eq!(result, ans);
    }

    #[test]
    fn test_tribonacci() {
        do_tribonacci_test(&[0., 1., 1.], 10, vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]);
        do_tribonacci_test(&[0., 1., 1.], 0, vec![]);
    }

    fn do_validate_pin_test(pin: &str, ans: bool) {
        let result = validate_pin(pin);
        assert!(result == ans);
    }

    #[test]
    fn test_validate_pin() {
        do_validate_pin_test("4567", true);
        do_validate_pin_test("56781", false);
        do_validate_pin_test("530434", true);
        do_validate_pin_test("135a", false);
        do_validate_pin_test("1.2345", false);
    }

    fn do_accum_test(s: &str, ans: &str) {
        let result = accum(s);
        assert_eq!(result.as_str(), ans);
    }

    #[test]
    fn test_accum() {
        do_accum_test("hello", "H-Ee-Lll-Llll-Ooooo");
    }

    fn do_to_camel_case_test(text: &str, ans: &str) {
        let result = to_camel_case(text);
        assert_eq!(result, ans);
    }

    #[test]
    fn to_camel_case_test() {
        do_to_camel_case_test("camel-case", "camelCase");
    }

    fn do_array_diff_test(a: Vec<i32>, b: Vec<i32>, ans: Vec<i32>) {
        let result = array_diff(a, b);
        assert_eq!(result, ans);
    }

    #[test]
    fn array_diff_test() {
        do_array_diff_test(vec![1,2], vec![1], vec![2]);
    }

    fn do_high_test(input: &str, ans: &str) {
        let result = high(input);
        assert_eq!(result, ans);
    }

    #[test]
    fn high_test() {
        do_high_test("a flying zebrazo", "zebrazo");
        do_high_test("aa b", "aa");
        do_high_test("b aa", "b");
    }
}

#![allow(dead_code)]
#![allow(unused)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    vec,
};

fn dna_strand(dna: &str) -> String {
    dna.chars()
        .map(|c| match c {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => unreachable!(),
        })
        .collect()
}

#[test]
fn test_dna_strand() {
    assert_eq!(dna_strand("AAAA"), "TTTT");
    assert_eq!(dna_strand("ATTGC"), "TAACG");
    assert_eq!(dna_strand("GTAT"), "CATA");
}

fn row_sum_odd_numbers(n: i64) -> i64 {
    n * n * n
}

#[test]
fn test_row_sum_odd_numbers() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(2), 8);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}

fn number(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops.iter().fold(0, |total, (x, y)| total + x - y)
}

#[test]
fn test_number() {
    assert_eq!(number(&[(10, 0), (3, 5), (5, 8)]), 5);
    assert_eq!(
        number(&[(3, 0), (9, 1), (4, 10), (12, 2), (6, 1), (7, 10)]),
        17
    );
    assert_eq!(
        number(&[(3, 0), (9, 1), (4, 8), (12, 2), (6, 1), (7, 8)]),
        21
    );
}

fn high_and_low(numbers: &str) -> String {
    let (max, min) = numbers
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .fold((i32::MIN, i32::MAX), |(max, min), n| {
            (if n > max { n } else { max }, if n < min { n } else { min })
        });
    format!("{} {}", max, min)
}

#[test]
fn test_high_and_low() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

fn persistence(num: u64) -> u64 {
    fn mul(n: u64) -> u64 {
        format!("{}", n)
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .fold(1, |p, x| p * x as u64)
    }

    let mut i = 0;
    let mut num = num;

    while num > 9 {
        num = mul(num);
        i += 1;
    }

    i
}

#[test]
fn test_persistence() {
    assert_eq!(persistence(39), 3);
    assert_eq!(persistence(4), 0);
    assert_eq!(persistence(25), 2);
    assert_eq!(persistence(999), 4);
}

fn longest(a1: &str, a2: &str) -> String {
    let mut concat = format!("{}{}", a1, a2).chars().collect::<Vec<_>>();
    concat.sort();
    concat.dedup();
    concat.iter().collect()
}

fn test_longest(s1: &str, s2: &str, exp: &str) -> () {
    println!("s1:{:?} s2:{:?}", s1, s2);
    println!("{:?} {:?}", longest(s1, s2), exp);
    println!("{}", longest(s1, s2) == exp);
    assert_eq!(&longest(s1, s2), exp)
}

#[test]
fn test_longest_2() {
    test_longest("aretheyhere", "yestheyarehere", "aehrsty");
    test_longest(
        "loopingisfunbutdangerous",
        "lessdangerousthancoding",
        "abcdefghilnoprstu",
    );
}

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let even = arr
        .into_iter()
        .cloned()
        .enumerate()
        .filter(|(pos, n)| n % 2 == 0)
        .collect::<Vec<_>>();

    let mut odds = arr
        .into_iter()
        .cloned()
        .filter(|&n| n % 2 != 0)
        .collect::<Vec<_>>();

    odds.sort_unstable();

    for (i, n) in even {
        odds.insert(i, n);
    }

    odds
}

#[test]
fn tset_sort_array() {
    assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
    assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
    assert_eq!(sort_array(&[]), []);
}

fn is_prime(x: i64) -> bool {
    if x <= 1 {
        return false;
    }
    if x == 2 {
        return true;
    }
    if x % 2 == 0 {
        return false;
    }

    let b = (x as f64).sqrt().floor() as i64;

    for i in (3..=b).step_by(2) {
        if x % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod is_prime_tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert!(!is_prime(0), "0 is not prime");
        assert!(!is_prime(1), "1 is not prime");
        assert!(is_prime(2), "2 is prime");
        assert!(is_prime(73), "73 is prime");
        assert!(!is_prime(75), "75 is not prime");
        assert!(!is_prime(-1), "-1 is not prime");
    }

    #[test]
    fn prime_tests() {
        assert!(is_prime(3), "3 is prime");
        assert!(is_prime(5), "5 is prime");
        assert!(is_prime(7), "7 is prime");
        assert!(is_prime(41), "41 is prime");
        assert!(is_prime(5099), "5099 is prime");
    }

    #[test]
    fn not_prime_tests() {
        assert!(!is_prime(4), "4 is not prime");
        assert!(!is_prime(6), "6 is not prime");
        assert!(!is_prime(8), "8 is not prime");
        assert!(!is_prime(9), "9 is not prime");
        assert!(!is_prime(45), "45 is not prime");
        assert!(!is_prime(-5), "-5 is not prime");
        assert!(!is_prime(-8), "-8 is not prime");
        assert!(!is_prime(-41), "-41 is not prime");
    }
}

fn alphabet_position(text: &str) -> String {
    text.to_ascii_uppercase()
        .chars()
        .filter_map(|c| {
            if c >= 'A' && c <= 'Z' {
                Some((c as u32 - 64).to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod test_alphabet_position {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(
            alphabet_position("The sunset sets at twelve o' clock."),
            "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
        );
        assert_eq!(
            alphabet_position("The narwhal bacons at midnight."),
            "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
        );
    }
}

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut seen = [false; 256];

    for x in ints {
        let y = s - x;
        if seen[y as u8 as usize] {
            return Some((y, *x));
        }
        seen[*x as u8 as usize] = true;
    }

    None
}

#[test]
fn test_sum_pairs() {
    let l1 = [1, 4, 8, 7, 3, 15];
    let l2 = [1, -2, 3, 0, -6, 1];
    let l3 = [20, -13, 40];
    let l4 = [1, 2, 3, 4, 1, 0];
    let l5 = [10, 5, 2, 3, 7, 5];
    let l6 = [4, -2, 3, 3, 4];
    let l7 = [0, 2, 0];
    let l8 = [5, 9, 13, -3];
    assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    assert_eq!(sum_pairs(&l3, -7), None);
    assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
}

fn spin_words(words: &str) -> String {
    words
        .split_whitespace()
        .map(|w| match w.len() {
            l if l > 4 => w.chars().rev().collect(),
            _ => w.to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod test_spin_words {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(
            spin_words("You are almost to the last test"),
            "You are tsomla to the last test"
        );
        assert_eq!(
            spin_words("Just kidding there is still one more"),
            "Just gniddik ereht is llits one more"
        );
        assert_eq!(
            spin_words("Seriously this is the last one"),
            "ylsuoireS this is the last one"
        );
    }
}

fn decode_morse(encoded: &str) -> String {
    let morse_code: HashMap<&'static str, String> = Default::default();

    encoded
        .trim()
        .split("   ")
        .map(|w| {
            w.split_whitespace()
                .map(|letter| morse_code.get(letter).unwrap().clone())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut substrings = arr_a
        .iter()
        .filter_map(|&a_str| {
            if arr_b
                .iter()
                .filter(|&b_str| b_str.len() >= a_str.len())
                .any(|&b_str| b_str.contains(a_str))
            {
                Some(a_str.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    substrings.sort();
    substrings.dedup();

    substrings
}

#[cfg(test)]
mod test_in_array {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            in_array(
                &["xyz", "live", "strong"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["live", "strong"]
        );

        assert_eq!(
            in_array(
                &["live", "strong", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );

        assert_eq!(
            in_array(
                &["tarp", "mice", "bull"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            [] as [&str; 0]
        );

        assert_eq!(
            in_array(
                &["live", "strong", "arp", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );
    }
}

fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h <= 0.0 || bounce < 0.0 || bounce >= 1.0 || window >= h {
        return -1;
    }

    let mut bounces = 1;
    let mut h = h;

    while h > window {
        // bounce
        h = h * bounce;

        if h > window {
            bounces += 2;
        }
    }

    bounces
}

fn testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
    assert_eq!(bouncing_ball(h, bounce, window), exp)
}

#[test]
fn tests_bouncing_ball() {
    testequal(3.0, 0.66, 1.5, 3);
    testequal(30.0, 0.66, 1.5, 15);
    testequal(40.0, 0.4, 10.0, 3);
    testequal(10.0, 0.6, 10.0, -1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn dir_reduc_loop(arr: &[Direction]) -> Vec<Direction> {
    let mut res = Vec::with_capacity(arr.len());

    for i in 0..arr.len() {
        match (res.last(), arr[i]) {
            (Some(Direction::North), Direction::South)
            | (Some(Direction::East), Direction::West)
            | (Some(Direction::West), Direction::East)
            | (Some(Direction::South), Direction::North) => {
                res.pop();
            }
            _ => res.push(arr[i]),
        }
    }

    res
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    arr.iter()
        .fold(Vec::new(), |mut res, next| match (res.last(), *next) {
            (Some(Direction::North), Direction::South)
            | (Some(Direction::East), Direction::West)
            | (Some(Direction::West), Direction::East)
            | (Some(Direction::South), Direction::North) => {
                res.pop();
                res
            }
            _ => {
                res.push(*next);
                res
            }
        })
}

#[cfg(test)]
mod dir_reduc_tests {
    use super::{
        dir_reduc,
        Direction::{self, *},
    };

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }
}

const LAST_DIGITS: [[i32; 5]; 10] = [
    [1, 0, 0, 0, 0], // 0
    [1, 1, 0, 0, 0], // 1
    [4, 6, 2, 4, 8], // 2
    [4, 1, 3, 9, 7], // 3
    [2, 6, 4, 0, 0], // 4
    [1, 5, 0, 0, 0], // 5
    [1, 6, 0, 0, 0], // 6
    [4, 1, 7, 9, 3], // 7
    [4, 6, 8, 4, 2], // 8
    [2, 1, 9, 0, 0], // 9
];

fn modulo(s: &str, n: i32) -> i32 {
    let offset = '0' as i32;
    let mut res = 0;
    for c in s.chars() {
        res = (res * 10 + c as i32 - offset) % n;
    }
    res
}

fn last_digit(str1: &str, str2: &str) -> i32 {
    if str2 == "0" {
        return 1;
    }

    let x_last_digit = &str1[str1.len() - 1..].parse::<usize>().unwrap();

    let divisor = LAST_DIGITS[*x_last_digit][0];
    let y_mod = modulo(str2, divisor) as usize;

    LAST_DIGITS[*x_last_digit][y_mod + 1]
}

#[test]
fn test_modulo() {
    assert_eq!(modulo("4", 4), 0);
    assert_eq!(modulo("14", 4), 2);
    assert_eq!(modulo("144", 4), 0);
    assert_eq!(modulo("1337", 42), 35);
    assert_eq!(modulo("123456789", 42), 15);
    assert_eq!(modulo("123456789654234", 42), 36);
    assert_eq!(
        modulo(
            "68819615221552997273737174557165657483427362207517952651",
            4
        ),
        3
    );
}

#[test]
fn test_last_digit() {
    assert_eq!(last_digit("4", "0"), 1);
    assert_eq!(last_digit("4", "1"), 4);
    assert_eq!(last_digit("4", "2"), 6);
    assert_eq!(last_digit("9", "7"), 9);
    assert_eq!(last_digit("10", "10000000000"), 0);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(
        last_digit(
            "3715290469715693021198967285016729344580685479654510946723",
            "68819615221552997273737174557165657483427362207517952651",
        ),
        7,
    );
    assert_eq!(
        last_digit(
            "542028",
            "84438841612238129190758601244058634992436629046705"
        ),
        8
    );
}

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    fn traverse_border(m: &[&[i32]]) -> Vec<i32> {
        let mut border = Vec::<i32>::new();
        let length = m.len();

        for col in 0..length {
            border.push(m[0][col]);
        }
        for row in 1..length {
            border.push(m[row][length - 1]);
        }
        for col in (0..length - 1).rev() {
            border.push(m[length - 1][col]);
        }
        for row in (1..length - 1).rev() {
            border.push(m[row][0]);
        }

        border
    }

    let mut snail = Vec::new();
    let mut it = 0;

    let len = if matrix.is_empty() {
        0
    } else {
        matrix[0].len()
    };

    let element_count = usize::pow(len, 2);

    while (snail.len() < element_count) {
        let inner_matrix = &matrix[it..len - it]
            .iter()
            .map(|row| &row[it..row.len() - it])
            .collect::<Vec<_>>();

        snail.extend(traverse_border(&inner_matrix));
        it += 1;
    }

    snail
}

#[cfg(test)]
mod snail_tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test5() {
        let square = &[];
        assert_eq!(snail(square), &[]);
    }
}

fn format_duration(seconds: u64) -> String {
    fn format(n: u64, singular: &str) -> Option<String> {
        if n > 0 {
            let suffix = if n > 1 {
                format!("{}s", singular)
            } else {
                singular.to_string()
            };
            return Some(format!("{} {}", n, suffix));
        }
        None
    }

    if seconds == 0 {
        return String::from("now");
    }

    let s = format(seconds % 60, "second");
    let m = format(seconds / 60 % 60, "minute");
    let h = format(seconds / (60 * 60) % 24, "hour");
    let d = format(seconds / (3600 * 24) % 365, "day");
    let y = format(seconds / (86400 * 365) % 365, "year");

    let parts = vec![y, d, h, m, s]
        .into_iter()
        .filter_map(|o| o)
        .collect::<Vec<_>>();

    match parts.len() {
        1 => format!("{}", parts[0]),
        _ => format!(
            "{} and {}",
            parts[0..parts.len() - 1].join(", "),
            parts[parts.len() - 1]
        ),
    }
}

#[cfg(test)]
mod test_format_duration {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(0), "now");
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
        assert_eq!(
            format_duration(15731080),
            "182 days, 1 hour, 44 minutes and 40 seconds"
        );
    }
}

fn rot13(message: &str) -> String {
    message
        .chars()
        .map(|c| match c {
            'a'..='z' => ((c as u8 - b'a' + 13) % 26 + b'a') as char,
            'A'..='Z' => ((c as u8 - b'A' + 13) % 26 + b'A') as char,
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod test_rot13 {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
    }
}

fn mix(s1: &str, s2: &str) -> String {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    enum Source {
        Left,
        Right,
        Both,
    }

    fn frequencies(s: &str, source: Source) -> Vec<(char, usize, Source)> {
        use std::collections::HashMap;
        s.chars()
            .filter(|c| c.is_lowercase())
            .fold(HashMap::new(), |mut map, c| {
                let count = map.entry(c).or_insert(0);
                *count += 1;
                map
            })
            .into_iter()
            .filter_map(|(c, count)| (count > 1).then(|| (c, count, source)))
            .collect::<Vec<_>>()
    }

    let mut freqs = frequencies(&s1, Source::Left);
    freqs.extend(frequencies(&s2, Source::Right));

    // 1: sort by char ascending and count descending
    freqs.sort_by(|(char1, count1, _), (char2, count2, _)| {
        char::cmp(char1, char2).then(usize::cmp(count1, count2).reverse())
    });

    // 2: deduplicate tuples with same char
    // - same count => pick first, update to Source::Both for sorting in next step
    // - diff count => pick first
    let mut freqs = freqs.into_iter().fold(Vec::new(), |mut stack, freq| {
        if stack.is_empty() {
            stack.push(freq);
        } else {
            match (stack.pop().unwrap(), freq) {
                ((char1, count1, _), (char2, count2, _)) if char1 == char2 && count1 == count2 => {
                    stack.push((char1, count1, Source::Both))
                }
                (f1 @ (char1, _, _), (char2, _, _)) if char1 == char2 => stack.push(f1),
                (f1, f2) => {
                    stack.push(f1);
                    stack.push(f2);
                }
            }
        }
        stack
    });

    // 3: sort by count descending, source ascending (to get equal counts in the same group) and char ascending
    freqs.sort_by(|(char1, count1, source1), (char2, count2, source2)| {
        usize::cmp(count1, count2)
            .reverse()
            .then(Source::cmp(source1, source2).then(char::cmp(char1, char2)))
    });

    // 4: convert to string
    freqs
        .into_iter()
        .map(|(c, count, source)| match source {
            Source::Both => format!("=:{}", c.to_string().repeat(count)),
            Source::Left => format!("1:{}", c.to_string().repeat(count)),
            Source::Right => format!("2:{}", c.to_string().repeat(count)),
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod test_mix {
    use super::mix;

    #[test]
    fn basics_mix() {
        testing(
            "Are they here",
            "yes, they are here",
            "2:eeeee/2:yy/=:hh/=:rr",
        );
        testing(
            "looping is fun but dangerous",
            "less dangerous than coding",
            "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg",
        );
        testing(
            " In many languages",
            " there's a pair of functions",
            "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt",
        );
        testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
        testing("codewars", "codewars", "");
        testing(
            "A generation must confront the looming ",
            "codewarrs",
            "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr",
        );
    }

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        assert_eq!(&mix(s1, s2), exp)
    }
}

fn recover_secret_yolo(triplets: Vec<[char; 3]>) -> String {
    fn find(a: &[char], n: char) -> Option<usize> {
        for (i, c) in a.iter().enumerate() {
            if *c == n {
                return Some(i);
            }
        }
        None
    }

    let mut res = vec![];

    loop {
        let mut converged = true;

        for t @ [a, b, c] in triplets.iter() {
            match (find(&res, *a), find(&res, *b), find(&res, *c)) {
                (None, None, None) => {
                    res.push(*a);
                    res.push(*b);
                    res.push(*c);
                    converged = false;
                }
                (None, None, Some(c_i)) => {
                    res.insert(c_i, *b);
                    res.insert(c_i, *a);
                    converged = false;
                }
                (None, Some(b_i), None) => {
                    res.insert(b_i, *a);
                    res.push(*c);
                    converged = false;
                }
                (None, Some(b_i), Some(c_i)) if b_i > c_i => {
                    res.remove(b_i);
                    res.insert(c_i, *b);
                    res.insert(c_i, *a);
                    converged = false;
                }
                (None, Some(b_i), Some(_)) => {
                    res.insert(b_i, *a);
                    converged = false;
                }
                (Some(_), None, None) => {
                    res.push(*b);
                    res.push(*c);
                    converged = false;
                }
                (Some(a_i), None, Some(c_i)) if a_i > c_i => {
                    res.remove(a_i);
                    res.insert(c_i, *b);
                    res.insert(c_i, *a);
                    converged = false;
                }
                (Some(a_i), None, Some(c_i)) => {
                    res.insert(c_i, *b);
                    converged = false;
                }
                (Some(a_i), Some(b_i), None) if a_i > b_i => {
                    res.remove(a_i);
                    res.insert(b_i, *a);
                    res.push(*c);
                    converged = false;
                }
                (Some(a_i), Some(b_i), None) => {
                    res.push(*c);
                    converged = false;
                }
                (Some(a_i), Some(b_i), Some(c_i)) if a_i > b_i => {
                    res.remove(a_i);
                    res.insert(b_i, *a);
                    converged = false;
                }
                (Some(a_i), Some(b_i), Some(c_i)) if b_i > c_i => {
                    res.remove(b_i);
                    res.insert(c_i, *b);
                    converged = false;
                }
                (Some(a_i), Some(b_i), Some(c_i)) => {}
            };
        }

        if converged {
            break;
        }
    }

    res.into_iter().collect()
}

// represents the input as a directed graph and applies
// topological sort to return the chars in order
fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    use std::collections::{HashMap, HashSet};

    type Graph = HashMap<char, HashSet<char>>;
    type InDegrees = HashMap<char, u32>;

    let (mut g, mut i) = triplets.iter().fold(
        (Graph::new(), InDegrees::new()),
        |(mut g, mut i), [a, b, c]| {
            i.entry(*a).or_insert(0);
            g.entry(*c).or_insert(HashSet::new());

            if g.entry(*a).or_insert(HashSet::new()).insert(*b) {
                *i.entry(*b).or_insert(0) += 1;
            }
            if g.entry(*b).or_insert(HashSet::new()).insert(*c) {
                *i.entry(*c).or_insert(0) += 1;
            }

            (g, i)
        },
    );

    let mut res = Vec::with_capacity(g.len());

    while !g.is_empty() {
        // get next node with in-degree = 0
        let next = i.iter().find(|(_, &v)| v == 0).unwrap().0.clone();

        // remove node from graph and decrement in-degree of neighbors
        g.remove(&next)
            .unwrap()
            .iter()
            .for_each(|out| *i.entry(*out).or_default() -= 1);

        // remove node from in-degrees
        i.remove(&next);

        res.push(next);
    }

    res.into_iter().collect()
}

#[test]
fn recover_secret_test() {
    assert_eq!(
        recover_secret(vec![
            ['t', 'u', 'p'],
            ['w', 'h', 'i'],
            ['t', 's', 'u'],
            ['a', 't', 's'],
            ['h', 'a', 'p'],
            ['t', 'i', 's'],
            ['w', 'h', 's']
        ]),
        "whatisup"
    );
}

fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut fib_n = 0;
    let mut fib_m = 1;

    while fib_n * fib_m < prod {
        fib_m = fib_n + fib_m;
        fib_n = fib_m - fib_n;
    }

    (fib_n, fib_m, fib_n * fib_m == prod)
}

fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}

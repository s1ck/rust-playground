#![allow(dead_code)]
#![allow(unused)]

use std::cmp::Ordering;

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

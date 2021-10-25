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

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.count {
            5 => None,
            _ => {
                self.count += 1;
                Some(self.count)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn counter_zipping() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(sum, 18)
    }

    #[test]
    fn iter() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None)
    }

    #[test]
    fn iter_sum() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        // consumes v1_iter
        // sum is a "consuming adaptor"
        let sum: i32 = v1_iter.sum();

        assert_eq!(sum, 6)
    }

    #[test]
    fn iter_owned() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.into_iter();

        assert_eq!(v1_iter.next(), Some(1));
        assert_eq!(v1_iter.next(), Some(2));
        assert_eq!(v1_iter.next(), Some(3));
        assert_eq!(v1_iter.next(), None)
    }

    #[test]
    fn iter_adaptor() {
        let v1 = vec![1, 2, 3];
        // iterator adaptor
        let map_iter = v1.iter().map(|x| x + 1);
        // consuming adaptor
        let sum: i32 = map_iter.sum();

        assert_eq!(sum, 9)
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes
            .into_iter()
            .filter(|shoe| shoe.size == shoe_size)
            .collect()
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let filtered = shoes_in_my_size(shoes, 10);

        println!("{:?}", filtered);

        assert_eq!(
            filtered,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        )
    }
}

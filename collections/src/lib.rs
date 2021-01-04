#[cfg(test)]
mod tests {
    
    use std::collections::HashMap;
    
    

    #[derive(Debug)]
    enum Element {
        Int(i32),
        Float(f32),
        String(String),
    }

    #[test]
    fn different_type_vec() {
        let v = vec![Element::Int(42), Element::Float(13.37), Element::String(String::from("foo"))];

        for e in &v {
            println!("{:?}", e);
        }
    }

    #[test]
    fn string_magic() {
        let mut s1 = String::from("foobar");
        let s2 = "barbaz".to_string();

        s1.push_str("baz");
        let _s2 = format!("{}foo", s2);
    }

    fn mean(a: &Vec<usize>) -> f64 {
        let mut sum = 0;

        for i in a {
            sum += i;
        }

        sum as f64 / a.len() as f64
    }

    fn median(a: &Vec<usize>) -> f64 {
        let mut copy = a.clone();
        copy.sort();

        let median: f64 = if a.len() % 2 == 1 {
            copy[a.len() / 2] as f64
        } else {
            let mid = a.len() / 2;
            (copy[mid] + copy[mid - 1]) as f64 / 2.0
        };

        median
    }

    fn mode(a: &Vec<usize>) -> usize {
        let mut m = HashMap::new();
        for i in a {
            let count = m.entry(i).or_insert(1);
            *count += 1;
        }

        let max_key = m.iter()
            .max_by_key(|&(_, v)| v)
            .map(|(k, _)| k)
            .expect("List was empty");

        **max_key
    }

    #[test]
    fn test_mean() {
        let a = vec![1, 2, 3, 4, 5];
        assert_eq!(3.0, mean(&a));
    }

    #[test]
    fn test_median() {
        let a = vec![1, 2, 3, 4, 5];
        assert_eq!(3.0, median(&a));
        let a = vec![1, 2, 3, 4, 4, 5];
        assert_eq!((3.0 + 4.0) / 2.0, median(&a));
    }

    #[test]
    fn test_mode() {
        let a = vec![1, 2, 3, 4, 4, 5];
        assert_eq!(4, mode(&a));
    }
}

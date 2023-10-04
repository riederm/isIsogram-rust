use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        let param = args.get(1).unwrap();
        if is_isogram(param) {
            println!("Yes! {param} is an isogram.");
        } else {
            println!("Nope! {param} is no isogram.");
        }
    }
}

pub fn is_isogram(value: &str) -> bool {
    // [0, 0, 0 ... 0] 255 times
    let mut counts = vec![0_u32; 255];
    // count all characters
    value.chars().for_each(|it| {
        counts[it as usize] += 1;
    });

    // see if we find different counts
    let mut max = None;
    for n in counts.iter().filter(|it| **it > 0) {
        match max {
            // if this is the first non-0 number we remember it
            None => max = Some(n),
            // we found a non-0 number that is different to what we remembered
            Some(m) if m != n => return false,
            // other cases are fine
            _ => {}
        }
    }
    // if we found a non-0 element and did not early exit it is an isogram
    max.is_some()
}

#[cfg(test)]
mod isogram_tests {
    use crate::is_isogram;

    #[test]
    fn empty_string_should_not_be_an_isogram() {
        assert!(!is_isogram(""));
    }

    #[test]
    fn one_letter_isograms() {
        assert!(is_isogram("a"));
        assert!(is_isogram("b"));
        assert!(is_isogram("x"));
        assert!(is_isogram("o"));
    }

    #[test]
    fn longer_letter_isograms() {
        assert!(is_isogram("aaaa"));
        assert!(is_isogram("bbbb"));
        assert!(is_isogram("xxxx"));
        assert!(is_isogram("oooo"));
    }

    #[test]
    fn longer_multiletter_isograms() {
        assert!(is_isogram("aaaa"));
        assert!(is_isogram("bbbb"));
        assert!(is_isogram("xxxx"));
        assert!(is_isogram("oooo"));
    }

    #[test]
    fn longer_multiletter_non_isograms() {
        assert!(!is_isogram("aabaa"));
        assert!(!is_isogram("bbabb"));
        assert!(!is_isogram("wxw"));
        assert!(!is_isogram("oor"));
    }

}

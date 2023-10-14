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
    //implementation goes here!
    false
}

#[cfg(test)]
mod isogram_tests {
    use crate::is_isogram;

    #[test]
    fn empty_string_should_not_be_an_isogram() {
        assert!(!is_isogram(""));
    }

    #[test]
    fn a_single_character_is_an_isogram(){
        assert!(is_isogram("a"));
    }
}

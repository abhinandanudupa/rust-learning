// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    let mut s = match chars.next() {
        None => String::new(),
        Some(first) => String::from(first).to_uppercase(),
    };
    s += &chars.collect::<String>();
    s
}

fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|x| capitalize_first(x)).collect()
}

fn capitalize_words_string(words: &[&str]) -> String {
    let ws = capitalize_words_vector(words);
    let mut out = String::new();
    for w in ws.iter() {
        out = out + w;
    }
    out
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}

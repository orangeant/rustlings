// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!

// Watch Later

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + &input[1..],
        // Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        // Some(first) => std::iter::once(first.to_ascii_uppercase()).chain(c).collect();
        // Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.into_iter().map(|x| capitalize_first(x)).collect()
    // words.iter().map(|x| capitalize_first(x)).collect()
    // let mut res = vec![];
    // for word in words.iter() {
    //     res.push(capitalize_first(word));
    // }
    // res
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|x| capitalize_first(x)).collect::<Vec<String>>().join("")
    // let mut res = String::new();
    // for word in words.iter() {
    //     res.push_str(&capitalize_first(word));
    // }
    // res
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
        let capitalized_words: Vec<String> = words.into_iter().map(capitalize_first).collect();
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");

        let capitalized_words = words.into_iter().map(capitalize_first).collect::<Vec<String>>().join("");
        assert_eq!(capitalized_words, "Hello World");
    }
}

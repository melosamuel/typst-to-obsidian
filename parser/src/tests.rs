#[cfg(test)]
mod tests {
    use crate::lorem::lorem_impl;

    #[test]
    fn test_zero_words() {
        assert_eq!(lorem_impl(0), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(lorem_impl(1), "Lorem.");
    }

    #[test]
    fn test_five_words() {
        assert_eq!(lorem_impl(5), "Lorem ipsum dolor sit amet.");
    }

    #[test]
    fn test_fifteen_words() {
        assert_eq!(
            lorem_impl(15),
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore."
        );
    }

    #[test]
    fn test_thirty_words() {
        assert_eq!(
            lorem_impl(30),
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim aeque doleamus animo, cum corpore dolemus, fieri."
        );
    }

    #[test]
    fn test_large_count_word_count() {
        let text = lorem_impl(1000);
        let words: Vec<&str> = text
            .split_whitespace()
            .filter(|&w| w != "\u{2013}")
            .collect();
        assert_eq!(words.len(), 1000);
        assert!(text.ends_with('.'));
    }
}

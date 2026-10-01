use lipsum::{LIBER_PRIMUS, LOREM_IPSUM, MarkovChain};
use serde::Serialize;
use std::{borrow::Cow, env, fs, sync::LazyLock};
use typst_syntax::{SyntaxKind, SyntaxNode, parse};

fn lorem_impl(n: usize) -> String {
    if n == 0 {
        return String::new();
    }

    static LOREM_CHAIN: LazyLock<MarkovChain<'static>> = LazyLock::new(|| {
        let mut chain = MarkovChain::new();
        chain.learn(LOREM_IPSUM);
        chain.learn(LIBER_PRIMUS);
        chain
    });

    let chain = &*LOREM_CHAIN;
    let mut iter = chain.iter_from(("Lorem", "ipsum"));
    const PUNCTUATION: [char; 3] = ['.', '!', '?'];

    let mut sentence = String::new();
    let mut word_count = 0;
    let mut needs_cap = false;

    while word_count < n {
        let Some(word) = iter.next() else { break };

        if word_count > 0 {
            sentence.push(' ');
        }

        if word == "--" {
            sentence.push('\u{2013}');
            continue;
        }

        if needs_cap {
            if let Some(c) = word.chars().next() {
                sentence.extend(c.to_uppercase());
                sentence.push_str(&word[c.len_utf8()..]);
            }
        } else {
            sentence.push_str(word);
        }

        needs_cap = sentence.ends_with(PUNCTUATION);
        word_count += 1;
    }

    if !sentence.ends_with(PUNCTUATION) {
        let idx = sentence
            .trim_end_matches(|c: char| c.is_ascii_punctuation())
            .len();
        sentence.truncate(idx);
        sentence.push('.');
    }

    sentence
}

fn extract_lorem_count(node: &SyntaxNode) -> Option<usize> {
    if node.kind() != SyntaxKind::FuncCall {
        return None;
    }
    let mut children = node.children();
    let first = children.next()?;
    if first.kind() != SyntaxKind::Ident || first.leaf_text() != "lorem" {
        return None;
    }
    let args = children.next()?;
    if args.kind() != SyntaxKind::Args {
        return None;
    }
    for arg_child in args.children() {
        if arg_child.kind() == SyntaxKind::Int {
            return arg_child.leaf_text().parse::<usize>().ok();
        }
    }
    None
}

#[derive(Serialize)]
struct JsonNode<'a> {
    kind: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    text: Cow<'a, str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<JsonNode<'a>>,
}

impl<'a> From<&'a SyntaxNode> for JsonNode<'a> {
    fn from(node: &'a SyntaxNode) -> Self {
        if let Some(count) = extract_lorem_count(node) {
            let generated = lorem_impl(count);
            return Self {
                kind: format!("{:?}", node.kind()),
                text: Cow::Owned(generated),
                children: node.children().map(JsonNode::from).collect(),
            };
        }

        Self {
            kind: format!("{:?}", node.kind()),
            text: Cow::Borrowed(node.leaf_text().as_str()),
            children: node.children().map(JsonNode::from).collect(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("usage: parser <filename>");
    let source = fs::read_to_string(filename).expect("failed to read input file");
    let parsed = parse(&source);
    let json = JsonNode::from(&parsed);
    serde_json::to_writer(std::io::stdout(), &json).expect("failed to serialize syntax tree");
}

#[cfg(test)]
mod tests {
    use super::*;

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

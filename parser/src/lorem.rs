use lipsum::{LIBER_PRIMUS, LOREM_IPSUM, MarkovChain};
use std::sync::LazyLock;
use typst_syntax::{SyntaxKind, SyntaxNode};

pub fn lorem_impl(n: usize) -> String {
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

pub fn extract_lorem_count(node: &SyntaxNode) -> Option<usize> {
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

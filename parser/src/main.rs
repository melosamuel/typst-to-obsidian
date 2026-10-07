mod lorem;

#[cfg(test)]
mod tests;

use serde::Serialize;
use std::{borrow::Cow, env, fs};
use typst_syntax::{SyntaxNode, parse};

use lorem::{extract_lorem_count, lorem_impl};

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

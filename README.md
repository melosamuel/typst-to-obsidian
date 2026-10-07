# Typst to Obsidian
This is a command-line Typst to Obsididan converter with Typst's native Rust parser and a converter written in Python. Currently a work in progress.

## Usage
`sh convert.sh filename.typ`

## Setup
1. `pip install -r render/requirements.txt`

## Testing

Run the Rust parser tests:

```sh
cd parser
cargo test
```

## Currently Supported Conversions

<details>
<summary>Text</summary>
  
- [x] Highlight
- [x] Line Break
- [x] Lorem
- [x] Lowercase
- [x] Overline
- [ ] Raw Text / Code
- [x] Small Capitals
- [ ] Smartquote
- [x] Strikethrough
- [x] Subscript
- [x] Superscript
- [x] Text
- [x] Underline
- [x] Uppercase

</details>

<details>
<summary>Math</summary>

- [ ] Accent

</details>

<details>
<summary>Symbols</summary>

- [ ] General Symbols

</details>

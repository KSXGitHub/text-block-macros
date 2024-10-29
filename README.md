# Text Block Macros

[![Test](https://github.com/KSXGitHub/text-block-macros/workflows/Test/badge.svg)](https://github.com/KSXGitHub/text-block-macros/actions?query=workflow%3ATest)
[![Crates.io Version](https://img.shields.io/crates/v/text-block-macros?logo=rust)](https://crates.io/crates/text-block-macros)

Create a multiline string literal.

## Usage Examples

### Create a block of text without final newline

```rust
use text_block_macros::text_block;
let text = text_block! {
  "abc"
  "def"
  "ghi"
};
assert_eq!(text, "abc\ndef\nghi");
```

### Create a block of text that ends with a newline

```rust
use text_block_macros::text_block_fnl;
let text = text_block_fnl! {
  "abc"
  "def"
  "ghi"
};
assert_eq!(text, "abc\ndef\nghi\n");
```

## License

[MIT](https://git.io/JYptO) © [Hoàng Văn Khải](https://github.com/KSXGitHub/).

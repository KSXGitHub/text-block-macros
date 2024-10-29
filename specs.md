# Specifications

## `text_block`

```rust
use text_block_macros::text_block;
assert_eq!(text_block! {}, "");
```

```rust
use text_block_macros::text_block;
assert_eq!(text_block! { "" }, "");
```

```rust
use text_block_macros::text_block;
assert_eq!(text_block! { "single line" }, "single line");
```

```rust
use text_block_macros::text_block;
assert_eq!(text_block! {
  "abc"
  "def"
  "ghi"
}, "abc\ndef\nghi");
```

## `text_block_fnl`

```rust
use text_block_macros::text_block_fnl;
assert_eq!(text_block_fnl! {}, "");
```

```rust
use text_block_macros::text_block_fnl;
assert_eq!(text_block_fnl! { "" }, "\n");
```

```rust
use text_block_macros::text_block_fnl;
assert_eq!(text_block_fnl! { "single line" }, "single line\n");
```

```rust
use text_block_macros::text_block_fnl;
assert_eq!(text_block_fnl! {
  "abc"
  "def"
  "ghi"
}, "abc\ndef\nghi\n");
```

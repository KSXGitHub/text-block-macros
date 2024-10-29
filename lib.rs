#![no_std]
#![no_implicit_prelude]

/// Create a multiline string literal.
///
/// **Example:**
///
/// ```
/// # #![no_implicit_prelude]
/// # use ::std::assert_eq;
/// use text_block_macros::text_block;
/// let text = text_block! {
///     "abc"
///     "def"
///     "ghi"
/// };
/// assert_eq!(text, "abc\ndef\nghi");
/// ```
#[macro_export]
macro_rules! text_block {
    () => {
        ""
    };

    ($line:literal) => {
        $line
    };

    ($head:literal $($tail:literal)*) => {
        ::std::concat!($head, $("\n", $tail,)*)
    };
}

/// Create a multiline string literal with a trailing newline.
///
/// **Example:**
///
/// ```
/// # #![no_implicit_prelude]
/// # use ::std::assert_eq;
/// use text_block_macros::text_block_fnl;
/// let text = text_block_fnl! {
///     "abc"
///     "def"
///     "ghi"
/// };
/// assert_eq!(text, "abc\ndef\nghi\n");
/// ```
#[macro_export]
macro_rules! text_block_fnl {
    ($($line:literal)*) => {
        ::std::concat!($($line, "\n",)*)
    };
}

// Tests for the sanitizer module
use super::*;

#[test]n test_sanitize() {
    let input = "Hello, World! @2023";
    let expected = "Hello World 2023";
    assert_eq!(sanitize(input), expected);
}
fnsanitize_empty_input() {
nlet input = "";
nlet expected = "";
asert_eq!(sanitize(input), expected);
d}
fntest_sanitize_special_characters() {
nlet input = "<script>alert('xss')</script>";
nlet expected = "scriptalertxss";
asert_eq!(sanitize(input), expected);
d}
fntest_sanitize_whitespace() {
nlet input = "   Extra Spaces   ";
nlet expected = "Extra Spaces";
asert_eq!(sanitize(input), expected);
d}

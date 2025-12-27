// Module for sanitizing input data

/// Sanitizes the given input string by removing unsafe characters.
///
/// # Arguments
/// 
/// * `input` - A string slice that holds the input to be sanitized.
///
/// # Returns
/// 
/// A new `String` containing the sanitized output.
pub fn sanitize(input: &str) -> String {
    // Performance optimization: Using a buffer to avoid multiple allocations
    let mut output = String::with_capacity(input.len());
    for c in input.chars() {
        if c.is_alphanumeric() || c == ' ' { // Allow alphanumeric and spaces
            output.push(c);
        }
    }
    output.trim().to_string() // Return trimmed result as a new String
}

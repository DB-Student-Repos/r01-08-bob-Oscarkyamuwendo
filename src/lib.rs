pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        "Fine. Be that way!"
    } else if message.to_uppercase() == message && message.chars().any(char::is_alphabetic) {
        "Whoa, chill out!"
    } else if message.trim().ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}

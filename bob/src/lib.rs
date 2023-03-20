pub fn reply(message: &str) -> &str {
    match message.trim() {
        str if str.ends_with("?") && str.to_uppercase() == str && str.chars().any(|c| c.is_uppercase()) => "Calm down, I know what I'm doing!",
        str if str.ends_with("?") => "Sure.",
        str if str.to_uppercase() == str && str.chars().any(|c| c.is_uppercase()) => "Whoa, chill out!",
        str if str.is_empty() => "Fine. Be that way!",
        _ => "Whatever.",
    }
}

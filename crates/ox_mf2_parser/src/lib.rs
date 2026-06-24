pub fn render_message(input: &str) -> String {
    format!("message: {input}")
}

#[cfg(test)]
mod tests {
    use super::render_message;

    #[test]
    fn renders_message() {
        assert_eq!(render_message("hello"), "message: hello");
    }
}

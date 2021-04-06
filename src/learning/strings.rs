pub fn my_name() -> &'static str {
    return "joe bloc";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_string() {
        assert_eq!(my_name(), "joe bloc");
    }

    #[test]
    fn test_format_string() {
        let string = format!("formatted string with name {}", my_name());
        assert_eq!(string, "formatted string with name joe bloc")

    }
}

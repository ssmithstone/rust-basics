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
}

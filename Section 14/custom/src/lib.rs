fn greeting(s: &str) -> String {
    format!("Hello {}", s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Bob");
        assert!(result.contains("Bob"),
            "Greeting did not contain name, value was {}", result
        );
    }
}

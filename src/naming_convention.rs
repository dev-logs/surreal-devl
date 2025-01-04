pub fn snake_case_to_camel(var_name: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in var_name.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

pub fn camel_to_snake_case(var_name: &str) -> String {
    let mut result = String::new();

    for (i, c) in var_name.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_snake_case_to_camel() {
        assert_eq!(snake_case_to_camel("hello_world"), "helloWorld");
        assert_eq!(snake_case_to_camel("another_example"), "anotherExample");
        assert_eq!(snake_case_to_camel("single"), "single");
        assert_eq!(snake_case_to_camel(""), "");
    }

    #[test]
    fn test_camel_to_snake_case() {
        assert_eq!(camel_to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(camel_to_snake_case("AnotherExample"), "another_example");
        assert_eq!(camel_to_snake_case("Single"), "single");
        assert_eq!(camel_to_snake_case(""), "");
        assert_eq!(camel_to_snake_case("camelCase"), "camel_case");
    }
}

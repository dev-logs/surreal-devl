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

pub fn to_string(value: &serde_json::Value) -> Option<String> {
    if let serde_json::Value::Array(array) = value {
        let mut lines = vec![];

        for item in array {
            if let serde_json::Value::String(string) = item {
                let line = string.as_str().trim();

                if !line.is_empty() {
                    lines.push(line);
                }
            } else {
                return None;
            }
        }

        let mut string = lines.join("\n");

        if !string.is_empty() {
            string.push('\n');
        }

        Some(string)
    } else {
        None
    }
}

#[test]
fn to_string_some() {
    assert_eq!(to_string(&serde_json::json!([])), Some(String::new()));
    assert_eq!(to_string(&serde_json::json!(["", ""])), Some(String::new()));
    assert_eq!(to_string(&serde_json::json!([" ", " "])), Some(String::new()));
    assert_eq!(to_string(&serde_json::json!(["foo", "bar", "baz"])), Some("foo\nbar\nbaz\n".to_owned()));
    assert_eq!(to_string(&serde_json::json!([" foo ", " bar ", " baz "])), Some("foo\nbar\nbaz\n".to_owned()));
    assert_eq!(to_string(&serde_json::json!(["\nfoo\n", "\nbar\n", "\nbaz\n"])), Some("foo\nbar\nbaz\n".to_owned()));
}

#[test]
fn to_string_none() {
    assert_eq!(to_string(&serde_json::json!({})), None);
    assert_eq!(to_string(&serde_json::json!([1])), None);
}

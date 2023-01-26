pub fn serialize(value: &serde_json::Value) -> Option<String> {
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

        lines.sort_unstable();

        let mut text = lines.join("\n");

        if !text.is_empty() {
            text.push('\n');
        }

        Some(text)
    } else {
        None
    }
}

#[test]
fn serialize_some() {
    assert_eq!(serialize(&serde_json::json!([])), Some(String::new()));
    assert_eq!(serialize(&serde_json::json!(["", ""])), Some(String::new()));
    assert_eq!(serialize(&serde_json::json!([" ", " "])), Some(String::new()));
    assert_eq!(serialize(&serde_json::json!(["foo", "bar", "baz"])), Some("bar\nbaz\nfoo\n".to_owned()));
    assert_eq!(serialize(&serde_json::json!([" foo ", " bar ", " baz "])), Some("bar\nbaz\nfoo\n".to_owned()));
    assert_eq!(serialize(&serde_json::json!(["\nfoo\n", "\nbar\n", "\nbaz\n"])), Some("bar\nbaz\nfoo\n".to_owned()));
}

#[test]
fn serialize_none() {
    assert_eq!(serialize(&serde_json::json!({})), None);
    assert_eq!(serialize(&serde_json::json!([1])), None);
}

pub fn to_string(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(string) => {
            let mut string = string.trim().to_owned();

            if !string.is_empty() {
                string.push('\n');
            }

            Some(string)
        },
        _ => None,
    }
}

#[test]
fn to_string_some() {
    assert_eq!(to_string(&serde_json::json!("")), Some(String::new()));
    assert_eq!(to_string(&serde_json::json!(" ")), Some(String::new()));
    assert_eq!(to_string(&serde_json::json!("foo")), Some("foo\n".to_owned()));
    assert_eq!(to_string(&serde_json::json!(" foo ")), Some("foo\n".to_owned()));
    assert_eq!(to_string(&serde_json::json!("\nfoo\n")), Some("foo\n".to_owned()));
    assert_eq!(to_string(&serde_json::json!("foo\n  bar")), Some("foo\n  bar\n".to_owned()));
}

#[test]
fn to_string_none() {
    assert_eq!(to_string(&serde_json::json!([""])), None);
    assert_eq!(to_string(&serde_json::json!(1)), None);
}

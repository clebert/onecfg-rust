pub fn merge(value_1: &mut serde_json::Value, value_2: serde_json::Value, array_merge: &ArrayMerge) {
    match (value_1, value_2) {
        (serde_json::Value::Array(array_1), serde_json::Value::Array(mut array_2)) => {
            match array_merge {
                ArrayMerge::Append => {
                    array_1.append(&mut array_2);
                }
                ArrayMerge::AppendUnique => {
                    for element_2 in array_2 {
                        if !array_1.contains(&element_2) {
                            array_1.push(element_2);
                        }
                    }
                }
                ArrayMerge::Replace => {
                    std::mem::drop(std::mem::replace(array_1, array_2));
                }
            };

            sort_values(array_1);
        }
        (serde_json::Value::Object(object_1), serde_json::Value::Object(object_2)) => {
            for entry in object_2 {
                let (key_2, value_2) = entry;

                if value_2 == serde_json::Value::Null {
                    object_1.remove(&key_2);
                } else if let Some(value_1) = object_1.get_mut(&key_2) {
                    merge(value_1, value_2, array_merge);
                } else {
                    object_1.insert(key_2, value_2);
                }
            }
        }
        (value_1, value_2) => {
            std::mem::drop(std::mem::replace(value_1, value_2));
        }
    }
}

fn sort_values(values: &mut [serde_json::Value]) {
    values.sort_by(|a, b| match (a, b) {
        (serde_json::Value::Bool(a), serde_json::Value::Bool(b)) => a.cmp(b),
        (serde_json::Value::Number(a), serde_json::Value::Number(b)) => {
            if let Some(a) = a.as_f64() {
                if let Some(b) = b.as_f64() {
                    if let Some(ordering) = a.partial_cmp(&b) {
                        return ordering;
                    }
                }
            }

            std::cmp::Ordering::Equal
        }
        (serde_json::Value::String(a), serde_json::Value::String(b)) => a.cmp(b),
        _ => std::cmp::Ordering::Equal,
    });
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArrayMerge {
    Append,
    AppendUnique,
    Replace,
}

impl Default for ArrayMerge {
    fn default() -> Self {
        Self::AppendUnique
    }
}

#[test]
fn merge_array_append_elements() {
    let mut value_1 = serde_json::json!([0, 1, 0]);
    let value_2 = serde_json::json!([3, 2, 1, 2]);

    merge(&mut value_1, value_2, &ArrayMerge::Append);
    assert_eq!(value_1, serde_json::json!([0, 0, 1, 1, 2, 2, 3]));
}

#[test]
fn merge_array_append_unique_elements() {
    let mut value_1 = serde_json::json!([0, 1, 0]);
    let value_2 = serde_json::json!([3, 2, 1, 2]);

    merge(&mut value_1, value_2, &ArrayMerge::AppendUnique);
    assert_eq!(value_1, serde_json::json!([0, 0, 1, 2, 3]));
}

#[test]
fn merge_array_replace() {
    let mut value_1 = serde_json::json!([0, 1]);
    let value_2 = serde_json::json!([3, 2]);

    merge(&mut value_1, value_2, &ArrayMerge::Replace);
    assert_eq!(value_1, serde_json::json!([2, 3]));
}

#[test]
fn merge_object_insert_entries() {
    let mut value_1 = serde_json::json!({"a": 0, "b": 1});
    let value_2 = serde_json::json!({"c": 2, "d": 3});

    merge(&mut value_1, value_2, &ArrayMerge::Append);
    assert_eq!(value_1, serde_json::json!({"a": 0, "b": 1, "c": 2, "d": 3}));
}

#[test]
fn merge_object_replace_entry() {
    let mut value_1 = serde_json::json!({"a": 0, "b": 1});
    let value_2 = serde_json::json!({"b": 2, "c": 3});

    merge(&mut value_1, value_2, &ArrayMerge::Append);
    assert_eq!(value_1, serde_json::json!({"a": 0, "b": 2, "c": 3}));
}

#[test]
fn merge_object_remove_entry() {
    let mut value_1 = serde_json::json!({"a": 0, "b": 1});
    let value_2 = serde_json::json!({"b": null, "c": 3});

    merge(&mut value_1, value_2, &ArrayMerge::Append);
    assert_eq!(value_1, serde_json::json!({"a": 0, "c": 3}));
}

#[test]
fn sort_values_empty() {
    let mut values: Vec<serde_json::Value> = vec![];

    sort_values(&mut values);

    assert_eq!(values, Vec::<serde_json::Value>::new());
}

#[test]
fn sort_values_boolean() {
    let mut values: Vec<serde_json::Value> =
        vec![serde_json::json!(true), serde_json::json!(false), serde_json::json!(true), serde_json::json!(false)];

    sort_values(&mut values);

    assert_eq!(
        values,
        vec![serde_json::json!(false), serde_json::json!(false), serde_json::json!(true), serde_json::json!(true)]
    );
}

#[test]
fn sort_values_number() {
    let mut values: Vec<serde_json::Value> = vec![serde_json::json!(42), serde_json::json!(0.1), serde_json::json!(7)];

    sort_values(&mut values);

    assert_eq!(values, vec![serde_json::json!(0.1), serde_json::json!(7), serde_json::json!(42)]);
}

#[test]
fn sort_values_string() {
    let mut values: Vec<serde_json::Value> =
        vec![serde_json::json!("foo"), serde_json::json!("bar"), serde_json::json!("baz")];

    sort_values(&mut values);

    assert_eq!(values, vec![serde_json::json!("bar"), serde_json::json!("baz"), serde_json::json!("foo")]);
}

#[test]
fn sort_values_object() {
    let mut values: Vec<serde_json::Value> =
        vec![serde_json::json!({"foo": "bar"}), serde_json::json!({"baz": "qux"}), serde_json::json!({})];

    sort_values(&mut values);

    assert_eq!(
        values,
        vec![serde_json::json!({"foo": "bar"}), serde_json::json!({"baz": "qux"}), serde_json::json!({})]
    );
}

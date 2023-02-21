pub fn normalize(path: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut normalized_path = std::path::PathBuf::new();
    let mut has_component = false;

    for component in path.components() {
        match component {
            std::path::Component::CurDir => (),
            std::path::Component::Normal(component) => {
                normalized_path.push(component);

                has_component = true;
            },
            _ => return None,
        }
    }

    if has_component {
        Some(normalized_path)
    } else {
        None
    }
}

#[test]
fn normalize_some() {
    use std::path::{Path, PathBuf};

    assert_eq!(normalize(Path::new("foo")), Some(PathBuf::from("foo")));
    assert_eq!(normalize(Path::new("foo/")), Some(PathBuf::from("foo")));
    assert_eq!(normalize(Path::new("./foo/.")), Some(PathBuf::from("foo")));
    assert_eq!(normalize(Path::new("foo/bar")), Some(PathBuf::from("foo/bar")));
    assert_eq!(normalize(Path::new("foo/bar/")), Some(PathBuf::from("foo/bar")));
    assert_eq!(normalize(Path::new("./foo/./bar/.")), Some(PathBuf::from("foo/bar")));
    assert_eq!(normalize(Path::new("foo/bar/baz")), Some(PathBuf::from("foo/bar/baz")));
    assert_eq!(normalize(Path::new("foo/bar/baz/")), Some(PathBuf::from("foo/bar/baz")));
    assert_eq!(normalize(Path::new("./foo/./bar/./baz/.")), Some(PathBuf::from("foo/bar/baz")));
}

#[test]
fn normalize_none() {
    use std::path::Path;

    assert_eq!(normalize(Path::new("")), None);
    assert_eq!(normalize(Path::new("/")), None);
    assert_eq!(normalize(Path::new("/foo")), None);
    assert_eq!(normalize(Path::new(".")), None);
    assert_eq!(normalize(Path::new("./")), None);
    assert_eq!(normalize(Path::new("./.")), None);
    assert_eq!(normalize(Path::new("..")), None);
    assert_eq!(normalize(Path::new("../")), None);
    assert_eq!(normalize(Path::new("../..")), None);
    assert_eq!(normalize(Path::new("../foo")), None);
    assert_eq!(normalize(Path::new("foo/..")), None);
    assert_eq!(normalize(Path::new("foo/../bar")), None);
}

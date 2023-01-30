# onecfg

One config file to generate them all.

## Installation

```
cargo install onecfg
```

## Usage

### Rust project

1. Create a config file (e.g. `onecfg.json`) with the following contents:

```json
{
  "extends": [
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/editorconfig.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/git.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/prettier.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/rust.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/vscode.json"
  ]
}
```

2. Run **onecfg**:

```
onecfg onecfg.json
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

# onecfg

One config file to generate them all.

## Installation

```
cargo install onecfg
```

## Usage

### Create a config file (e.g. `onecfg.json`)

#### Rust project

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

#### TypeScript project

```json
{
  "extends": [
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/editorconfig.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/eslint.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/git.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/jest.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/node.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/prettier.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/swc.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/typescript.emit.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/typescript.eslint.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/typescript.json",
    "https://raw.githubusercontent.com/clebert/onecfg-rust/main/example/vscode.json"
  ]
}
```

### Run onecfg

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

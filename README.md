# onecfg

One config file to generate them all.

## Installation

```
cargo install onecfg
```

## Usage

### Create a config file (e.g. `onecfg.json`)

Predefined [config files](https://github.com/clebert/onecfg-lib) can be extended
for a quick start:

```json
{
  "extends": [
    "https://raw.githubusercontent.com/clebert/onecfg-lib/main/lib/onecfg-editorconfig.json",
    "https://raw.githubusercontent.com/clebert/onecfg-lib/main/lib/onecfg-git.json",
    "https://raw.githubusercontent.com/clebert/onecfg-lib/main/lib/onecfg-prettier.json",
    "https://raw.githubusercontent.com/clebert/onecfg-lib/main/lib/onecfg-vscode.json"
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

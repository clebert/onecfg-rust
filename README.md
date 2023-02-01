# onecfg

> One config file to generate them all.

A program for managing config files across multiple projects with the
flexibility to extend and customize
[predefined](https://github.com/clebert/onecfg-lib) configurations. The idea is
that a single config file can be used to control and generate all the necessary
settings for a repository, reducing the need to manage multiple files or
manually configure settings each time a project is set up or modified.

## Installation

```
cargo install onecfg
```

## Usage

In short, a onecfg file (e.g. `onecfg.json`) allows for the automatic generation
of config files as follows:

```
onecfg onecfg.json
```

### Defining the format of the config files to be generated

```json
{
  "defines": {
    ".prettierrc.json": {"format": "json"}
  }
}
```

### Declaring patches specific to certain config files

```json
{
  "patches": {
    ".prettierrc.json": [
      {"value": {"printWidth": 80}},
      {"value": {"singleQuote": true}}
    ]
  }
}
```

### Extending onecfg files, including using [predefined](https://github.com/clebert/onecfg-lib) ones for convenience

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

_Note: You can use
[JSON Schema](https://github.com/clebert/onecfg-rust/blob/main/schema.json) to
validate your onecfg file or enable autocompletion in the editor._

## Config formats

### `text`

```json
{
  "defines": {
    "test.txt": {"format": "text"}
  },
  "patches": {
    "test.txt": [{"value": "foo"}, {"value": "bar"}]
  }
}
```

```
bar
```

### `json`

```json
{
  "defines": {
    "test.json": {"format": "json"}
  },
  "patches": {
    "test.json": [{"value": {"foo": "bar"}}, {"value": {"baz": "qux"}}]
  }
}
```

```json
{
  "baz": "qux",
  "foo": "bar"
}
```

### `toml`

```json
{
  "defines": {
    "test.toml": {"format": "toml"}
  },
  "patches": {
    "test.toml": [{"value": {"foo": "bar"}}, {"value": {"baz": "qux"}}]
  }
}
```

```toml
baz = "qux"
foo = "bar"
```

### `yaml`

```json
{
  "defines": {
    "test.yml": {"format": "yaml"}
  },
  "patches": {
    "test.yml": [{"value": {"foo": "bar"}}, {"value": {"baz": "qux"}}]
  }
}
```

```yaml
baz: qux
foo: bar
```

### `ignorefile`

```json
{
  "defines": {
    ".testignore": {"format": "ignorefile"}
  },
  "patches": {
    ".testignore": [{"value": ["foo", "bar"]}, {"value": ["baz"]}]
  }
}
```

```
bar
baz
foo
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

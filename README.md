# onecfg

> One config file to generate them all.

`onecfg` provides an efficient solution for managing config files in projects
that use multiple tools with interdependent configurations. It introduces the
concept of **onecfg files**, which are JSON-based files that store the necessary
information to generate and customize config files for different tools. These
onecfg files can be shared, extended, and loaded via HTTPS from any location,
enabling users to create customized configurations tailored to their specific
needs.

This approach addresses two main challenges: first, it eliminates the repetitive
nature of config files and reduces the need for manual updates across projects.
Second, it simplifies the complexity of managing configurations when working
with a combination of different tools.

## Installation

You can install `onecfg` using Cargo:

```
cargo install onecfg
```

## Usage

Before you can generate config files with `onecfg`, you'll need to create a
`onecfg.json` file in your project's root directory. This file will define how
the config files should be generated, as well as any customizations you'd like
to make.

To generate the config files, use the following command:

```
onecfg onecfg.json
```

To quickly get started with `onecfg`, you can use the
[`onecfg-lib`](https://github.com/clebert/onecfg-lib) library. It is a
collection of onecfg files specifically designed to configure TypeScript and
Rust projects. By extending these predefined onecfg files, you can easily set up
your project without having to create your own configurations from scratch.

To use the `onecfg-lib` library, simply include the desired onecfg files in the
`extends` section of your project's `onecfg.json` file. For example:

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

In this example, the `onecfg.json` file extends four different onecfg files from
the `onecfg-lib` library, which are specifically designed for configuring
EditorConfig, Git, Prettier, and Visual Studio Code. By extending these files,
your project will be set up with the recommended configuration for each tool.

You can find more onecfg files for TypeScript and Rust projects in the
[`onecfg-lib` repository](https://github.com/clebert/onecfg-lib).

_Note: You can use the
[JSON Schema](https://github.com/clebert/onecfg-rust/blob/main/schema.json) to
validate your onecfg file or enable autocompletion in the editor._

## Defining the format of the config files to be generated

In your `onecfg.json` file, you can define the format of the config files as
shown below:

```json
{
  "defines": {
    ".gitignore": {"format": "ignorefile"},
    ".prettierrc.json": {"format": "json"}
  }
}
```

It's important to note that only config files directly or indirectly defined in
an extended onecfg file can be generated and patched. A config file under a
specific path should only be defined once across all extended onecfg files.

When extending onecfg files, ensure that there are no conflicting definitions
for the same file paths. If a conflict occurs, you may need to either modify
your custom `onecfg.json` file or create a new onecfg file to resolve the issue.

### Declaring patches specific to certain config files

You can declare patches specific to certain config files in your `onecfg.json`
file:

```json
{
  "patches": {
    ".gitignore": [{"value": ["/dist"]}],
    ".prettierrc.json": [{"value": {"printWidth": 80, "singleQuote": true}}]
  }
}
```

It's essential to understand that patches for undefined files will have no
effect and won't negatively impact the configuration process. This has the
advantage of allowing you to plan integration patches for tools that may not be
used in a specific project. If the tool is not being used, the corresponding
configuration won't be applied, thus preventing any unnecessary configurations
from being generated.

## Config formats

`onecfg` supports various config formats including `text`, `json`, `toml`,
`yaml`, and `ignorefile`. You can define the format of a config file in the
defines section of your onecfg.json file, and provide patches to customize the
values.

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
foo
bar
baz
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

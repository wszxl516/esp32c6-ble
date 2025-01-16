<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

# vivi

`vivi` is a component library for [Slint](https://slint.dev/).

`vivi` provides currently the following two sets of components:

- `foundation`: Base components that can be used to create a custom component set.
- `magic`: Ready to use component set with a custom design based on [Catppuccin](https://github.com/catppuccin/catppuccin).

<a href="https://slint.dev">
    <img alt="#MadeWithSlint" src="https://raw.githubusercontent.com/slint-ui/slint/master/logo//MadeWithSlint-logo-light.svg" height="60">
</a>

[![Crates.io](https://img.shields.io/crates/v/vivi)](https://crates.io/crates/vivi_ui)
[![Rust docs 0.2.0](https://img.shields.io/badge/docs.rust-v0.2.0-orange.svg)](https://vivi-ui.codeberg.page/@main/vivi/0.2.0/docs/rust/vivi_ui)
[![Slint docs 0.2.0](https://img.shields.io/badge/docs.slint-0.2.0-blue.svg)](https://vivi-ui.codeberg.page/@main/vivi/0.2.0/docs/slint/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSES/MIT.txt)

## Examples

There is the [Gallery](../examples/gallery) that contains an overview of all `magic` components `vivi` provides.

## Prerequisites

- `Slint` >= 1.8

## How to use with Rust

1. Add `vivi_ui` as build dependency to your `Cargo.toml`:

```toml
[dependencies]
slint = { version = "1.8" }

[build-dependencies]
slint-build = { version = "1.7" }
vivi_ui = { version = "0.2" }
```

2. Use `vivi::import_paths()` in your `build.rs` file. It will make coop's files visible as `@vivi`.

```rust
fn main() {
    slint_build::compile(
        "ui/index.slint",
        slint_build::CompilerConfiguration::new()
            .with_library_paths(vivi_ui::import_paths()),
    ).unwrap();
}
```

3. Add an import to your Slint file (`ui/index.slint`):

```slint
import { MagicWindow, FilledButton } from "@vivi/magic.slint";

export component MyApp inherits MagicWindow {
    preferred-width: 600px;
    preferred-height: 400px;
    title: "MyApp";

    FilledButton {
        text: "Click me";
    }
}
```

## Get Started

To quickly get started, you can use the following rust template repository:

- [vivi template](https://app.radicle.xyz/nodes/ash.radicle.garden/rad:z3xAtPtoBbxAza3RRJDqaNHVzju9v)

## Contribution

Ideas, feedback and code contributions are welcome. Please check the [Contribution Guide](https://app.radicle.xyz/nodes/seed.radicle.garden/rad:z3oxAZSLcyXgpa7fcvgtueF49jHpH/tree/CONTRIBUTING.md) for more details.

## License

The source code of `vivi` and examples are available under [MIT license](../LICENSES/MIT.txt).

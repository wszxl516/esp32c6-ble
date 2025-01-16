// SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
// SPDX-License-Identifier: MIT

/*!
 * # vivi
 *
 * `vivi` is a custom component library for [Slint](https://slint.dev/).
 *
 * ## How to use
 *
 * First of all you have to add `slint` as dependency to the `Cargo.toml` file of your project and `vivi_ui` and `slint-build` as build
 * dependency.
 *
 * ### Cargo.toml
 *
 * ```toml
 * [package]
 * ...
 * build = "build.rs"
 * edition = "2021"
 *
 * [dependencies]
 * slint = { version = "1.7" }
 * ...
 *
 * [build-dependencies]
 * slint-build = { version = "1.7" }
 * vivi_ui = "0.2.0"
 * ```
 * As next use the API of the slint-build crate in the `build.rs` file and vivi to include the import paths.
 *
 * ### build.rs
 *
 * ```ignore
 * fn main() {
 *   slint_build::compile_with_config(
 *          "ui/index.slint",
 *          slint_build::CompilerConfiguration::new().with_library_paths(vivi_ui::import_paths()),
 *      )
 *      .unwrap();
 * }
 * ```
 *
 * `vivi` can now be used inside of your Slint files:
 *
 * ### index.slint
 *
 * ```slint
 * import { MagicVerticalBox, FilledButton } from "@vivi/magic.slint";
 *
 * export component HelloWorld inherits Window {
 *     MagicVerticalBox {
 *         FilledButton {
 *             text: "Click Me";
 *             clicked => { self.text = "Hello World"; }
 *         }
 *    }
 * }
 * ````
 *
 * Finally, use the [`include_modules!`] macro in your `main.rs`:
 *
 * ### main.rs
 *
 * ```ignore
 * slint::include_modules!();
 * fn main() {
 *     HelloWorld::new().unwrap().run().unwrap();
 * }
 * ```
 *
 */

use std::env;
use std::path::PathBuf;

use std::collections::HashMap;

const LIB_NAME: &str = "vivi";

/// Provides the `vivi` library paths used by slint-build to make them accessible through `@vivi` in Slint.
///
/// ```ignore
/// fn main() {
///   slint_build::compile_with_config(
///          "ui/index.slint",
///          slint_build::CompilerConfiguration::new().with_library_paths(vivi::import_paths()),
///      )
///      .unwrap();
/// }
/// ```
///
/// Import `vivi` library in `ui/main.slint`
///
/// ```slint
/// import { FilledButton } from "@vivi/magic.slint";
/// ````
pub fn import_paths() -> HashMap<String, PathBuf> {
    let mut import_paths = HashMap::new();

    let ui_lib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui");

    import_paths.insert(LIB_NAME.to_string(), ui_lib_path);

    import_paths
}

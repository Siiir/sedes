# Sedes

## 🎯 Overview

An open-source Rust library, which focuses on doing **serialization** and **deserialization** **with {dynamic, deduced} serialization format**.

Currently, supported deserialization formats - `JSON`, `YAML`, `CBOR`, `RMP`, `Bincode`, `Pickle`.  
Currently, supported serialization formats - `JSON`, `Pretty JSON`, `YAML`, `CBOR`, `RMP`, `Bincode`, `Pickle`.  

## Example use case

**Behold the deduction of (de)serialization format** from file extension.

### Deserialization

Write JSON to a file, then deserialize.

```rust
let path = std::env::temp_dir().join("example.json");
std::fs::write(&path, "[1, 2, 42]").unwrap();
let deserialized: Vec<i32> = sedes::deserialize_from_file(&path).unwrap();
assert_eq!(deserialized, vec![1, 2, 42]);
```

### Serialization

Write to a temporary yaml file, then assert content.

```rust
let path = std::env::temp_dir().join("example.yml");
sedes::serialize_to_file( &path, "W", &[1, 2, 42] ).unwrap();
let serialized = std::fs::read_to_string(&path).unwrap();
assert_eq!(serialized, "- 1\n- 2\n- 42\n");
```

## 📋 Prerequisites

- 📦 `cargo` including `fmt` subcommand

## 🔄 Continuous Integration

This project uses GitHub Actions for continuous integration. The CI pipeline includes:

- ✅ Running all tests (`cargo test --all-targets`)
- 📚 Documentation checks (`cargo rustdoc`)
- 🤖 Automatic formatting fixes with `cargo fmt`
- 💅 Code formatting check `cargo fmt --check`
- ⏪ Automatic reversion of commits that fail CI checks

### 🧹 Code Style Guide (rustfmt)

This project uses a custom [rustfmt](https://github.com/rust-lang/rustfmt) configuration to enforce consistent code formatting. Below are the key formatting rules we are following:

#### 🛠️ Formatting Rules

| Setting                          | Value       | Description                                        |
| -------------------------------- | ----------- | -------------------------------------------------- |
| `max_width`                      | `100`       | Maximum width of a line before breaking            |
| `hard_tabs`                      | `false`     | Use spaces instead of tabs                         |
| `tab_spaces`                     | `4`         | Number of spaces per indentation level             |
| `newline_style`                  | `"Auto"`    | Use system-native line endings (LF/CRLF)           |
| `use_small_heuristics`           | `"Default"` | Enables formatting heuristics for small constructs |
| `fn_call_width`                  | `60`        | Max width for function calls on a single line      |
| `attr_fn_like_width`             | `70`        | Max width for attribute-like function macros       |
| `struct_lit_width`               | `18`        | Max width for struct literals                      |
| `struct_variant_width`           | `35`        | Max width for struct variants in enums             |
| `array_width`                    | `60`        | Max width for arrays on a single line              |
| `chain_width`                    | `60`        | Max width for method chains on a single line       |
| `single_line_if_else_max_width`  | `50`        | Max width for single-line `if/else` expressions    |
| `single_line_let_else_max_width` | `50`        | Max width for single-line `let...else` expressions |
| `reorder_imports`                | `true`      | Automatically sort `use` statements                |
| `reorder_modules`                | `true`      | Automatically sort module declarations             |
| `fn_params_layout`               | `"Tall"`    | Format function params vertically (one per line)   |
| `edition`                        | `2024`      | Rust edition for parsing and formatting            |
| `style_edition`                  | `2024`      | Edition used for formatting style rules            |

### 🧪 Formatting Check in CI

Formatting is checked in CI using:

```bash
cargo fmt --check
```

Run `cargo fmt` before push to automatically format your code.
**The CI workflow will fail if your code does not follow the formatting rules.**

### Reset in the CI

If the workflow fails, the branch will be reset to state before this commit using `git reset --hard HEAD~1` <br />
**How to work after this on the local environment?**

- If you want to discard all changes from the failing commit, run `git pull --rebase`
- If you want to introduce some fixes, just create another commit and push it to try one more time

## 🤝 Contributing Guideline

1. 🍴 **`git clone` the repository**
2. 🌿 **Create new branch from main** (`git checkout -b <branch-name>`)
3. ✏️ **Make your changes** (`git add .; git commit -m <msg>`)  
   **Remember** about subissue closing. ("\<msg\>. Closes #K")
4. 🧪 **Run the tests** (`cargo test`)
5. 💅 **Ensure your code is formatted** (`cargo fmt`)
6. 💾 **Commit your changes** (`git commit -m 'Add some amazing feature'`)
7. 📤 **Push to the branch** (`git push origin <branch-name>`)
8. 🔄 **Open a Merge/Pull Request**  
   **Remember** about closing the MR's target issue "\<description\>.\n Closes #N"

### 🎯 Issue Closing

When working on issues, follow these guidelines:

- **Merge/Pull Requests** should target and close the main issue they're addressing:
- **Individual commits** should close sub-issues or tasks:

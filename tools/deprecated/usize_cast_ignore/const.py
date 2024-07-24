AST_COMMAND = "rustc -Z unpretty=ast-tree"
DISABLE_LINT = '#[allow(clippy::cast_possible_truncation)]\n'
CLIPPY_COMMAND = 'cargo --color never clippy --target x86_64-unknown-none --manifest-path charlotte_core/Cargo.toml --message-format json'
LINT_REASON = 'compiler-message'
LOOK_FOR = 'warning: casting `u64` to `usize` may truncate the value'

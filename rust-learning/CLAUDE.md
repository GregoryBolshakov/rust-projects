# rust-learning

Sandbox project for practicing Rust. Mirrors the quality-of-life setup used in
the adjacent C++ projects (interview-prep-cpp-systems, CPP_Project) but built
around Cargo and rust-analyzer.

## Layout

```
rust-learning/
├── .vscode/
│   ├── extensions.json   # recommended extensions (rust-analyzer, CodeLLDB, ...)
│   ├── launch.json       # Debug + Run configurations (gdb and CodeLLDB)
│   ├── settings.json     # rust-analyzer + format-on-save tuning
│   └── tasks.json        # cargo build / run / test / check / clippy / fmt / clean
├── src/
│   ├── main.rs           # default binary — sandbox area
│   └── bin/              # one file = one runnable practice binary
│       ├── 01_variables.rs
│       └── 02_ownership.rs
├── tests/                # integration tests (one file per scenario)
├── Cargo.toml            # crate manifest
├── clippy.toml           # clippy thresholds
├── rustfmt.toml          # formatter config (matches the 120-col house style)
└── .gitignore
```

## Common workflows

- **Type-check everything fast**: `cargo check --all-targets`
- **Build default binary**: `cargo build`
- **Run default binary**: `cargo run`
- **Run a specific exercise**: `cargo run --bin 01_variables`
- **Run all tests**: `cargo test`
- **Lint with pedantic clippy**: `cargo clippy --all-targets -- -W clippy::pedantic`
- **Format**: `cargo fmt`
- **Open API docs**: `cargo doc --no-deps --open`

All of these are also wired up as VS Code tasks (Ctrl+Shift+P → "Run Task").
The default build task (Ctrl+Shift+B) is `cargo: build`. The default test task
is `cargo: test`.

## Adding a new practice exercise

1. Drop a new `<name>.rs` into `src/bin/` with its own `fn main()`.
2. Open the file. Either click rust-analyzer's `Run | Debug` CodeLens above
   `fn main`, or use the "Debug current bin" launch config (F5).
3. To run from the terminal: `cargo run --bin <name>`.

Each file under `src/bin/` becomes its own binary automatically — no need to
edit `Cargo.toml`.

## Debugging

Two debugger paths are configured:

- **gdb (already installed on this machine)** — the `Debug main (gdb)` and
  `Debug current bin (gdb)` configs use the `rust-gdb` wrapper that ships with
  rustup, which preloads Rust pretty-printers.
- **CodeLLDB (recommended for Rust)** — install the `vadimcn.vscode-lldb`
  extension (it bundles its own LLDB). The CodeLLDB launch configs use Cargo's
  built-in target resolution, so you don't have to maintain binary paths.

`RUST_BACKTRACE=1` is set in both, so panics print full stack traces.

## Toolchain

Installed via rustup at `~/.cargo/`:

- rustc + cargo (stable)
- rustfmt
- clippy
- rust-analyzer
- rust-gdb / rust-lldb wrappers

`source ~/.cargo/env` is appended to `~/.bashrc` and `~/.profile` by rustup.
If `cargo` isn't on PATH in a new shell, run `source ~/.cargo/env` first.

## House style

- 4-space indent (Rust convention; rustfmt enforces).
- 120-column soft wrap — set in `rustfmt.toml` and hinted by the editor ruler.
- Format on save is on for `.rs` files via rust-analyzer.
- Clippy is wired into rust-analyzer's "check on save" so warnings show inline.

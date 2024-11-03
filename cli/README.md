# cli

A command line utility tool to enable CI/CD. This command runs features of `ris_engine` as a oneshot cli program and provides further utility functions.

## Usage

Change to the root directory of this repository and interface with this program via cargo and [its package selection](https://doc.rust-lang.org/cargo/commands/cargo-run.html#package-selection).

For all available commands run:

```bash
cargo run -p cli
```

To get further information on a specific command and what additional args it may be need, run:

```bash
cargo run -p cli help <command>
```

To execute a command, run:

```bash
cargo run -p cli <command> [args...]
```

For example, the command below runs the testing pipeline:

```bash
cargo run -p cli pipeline all
```
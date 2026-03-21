# skilib

`skilib` is a Rust-based toolkit that wraps and validates external CLI binaries or codebases so they can be consumed consistently across the ski.ai ecosystem.

## What it does

- Wraps a binary + source path into a portable skill manifest.
- Validates that the wrapped artifact is structurally sound.
- Provides a small, deterministic CLI interface for automation pipelines.

## Commands

- `skilib wrap --name <skill-name> --binary <path> --codebase <path> [--out <manifest-path>]`
- `skilib validate --manifest <manifest-path>`
- `skilib inspect --manifest <manifest-path>`

## Example

```bash
cargo run -- wrap --name ski-search --binary ./bin/ski-search --codebase ./skills/ski-search --out ./skill.manifest
cargo run -- validate --manifest ./skill.manifest
cargo run -- inspect --manifest ./skill.manifest
```

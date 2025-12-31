# pyrope

A proof-of-concept library that brings Rust's `Result` and `Option` types to Python, treating failures as values instead of exceptions.

The concept: "Drop a rope (type safety from Rust) into the dangerous freedom of Python."

## Quick Example

```python
from pyrope import Blueprint, Op, run

# Define a pipeline
bp = (
    Blueprint()
    .pipe(Op.split(","))
    .pipe(Op.index(0))
    .pipe(Op.expect_str())
    .pipe(Op.to_uppercase())
)

# Execute with type-safe error handling
result = run(bp, "hello,world")
if result.is_ok():
    print(result.unwrap())  # "HELLO"
else:
    print(f"Error: {result.unwrap_err().message}")
```

### Generator-based short-circuiting (Rust `?` operator style)

```python
from pyrope import Ok, Result, do

@do
def process(value: str) -> Result[str, object]:
    text = yield Ok(value)  # Type checkers infer 'text' as str
    upper = yield Ok(text.upper())
    return Ok(f"Processed: {upper}")

print(process("hello").unwrap())  # "Processed: HELLO"
```

## Why Not Exceptions?

1. **Explicit control flow**: Treat failures as values, not control flow jumps
2. **No implicit None**: Force explicit `unwrap()` or `is_some()` checks
3. **Rust-like short-circuiting**: Reproduce Rust's `?` operator in Python using generators

## Type Checker Support

- **Pyright**: Primary focus - verifies that `yield` correctly infers types
- **MyPy**: Strict mode compatibility verified

## Installation

### Requirements

- Python 3.12+
- Rust toolchain (pinned via `rust-toolchain.toml`)
- [uv](https://github.com/astral-sh/uv)
- [cargo-make](https://github.com/sagiegurari/cargo-make)

### Setup

```bash
# Install dependencies and build extension
uv sync
makers dev

# Or run everything at once
makers ci
```

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, code generation system, and testing guidelines.

## License

This is a proof-of-concept project and is not intended for production use.

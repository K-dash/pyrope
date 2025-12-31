# pyrope

A proof-of-concept library that brings Rust's `Result` and `Option` types to Python, treating failures as values instead of exceptions.

The concept: "Drop a rope (type safety from Rust) into the dangerous freedom of Python."

## Why Not Exceptions?

1. **Explicit control flow**: Treat failures as values, not control flow jumps
2. **No implicit None**: Force explicit `unwrap()` or `is_some()` checks
3. **Rust-like short-circuiting**: Reproduce Rust's `?` operator in Python using generators

## Direct Usage

You can use `Result` and `Option` types directly for manual handling or functional chaining, just like in Rust.

### Manual Handling

```python
from pyrope import Ok, Err, Result

def divide(a: int, b: int) -> Result[float, str]:
    if b == 0:
        return Err("Division by zero")
    return Ok(a / b)

res = divide(10, 2)
if res.is_ok():
    print(f"Success: {res.unwrap()}")  # Success: 5.0
else:
    print(f"Failure: {res.unwrap_err()}")
```

### Functional Chaining (`map`, `and_then`)

Avoid `if` checks by chaining operations.

```python
from pyrope import Ok

res = (
    Ok("123")
    .map(int)                # Result[int, E]
    .map(lambda x: x * 2)    # Result[int, E]
    .and_then(lambda x: Ok(f"Value is {x}"))
)
print(res.unwrap())  # "Value is 246"
```

> **Type Hint for `and_then`**: When using `and_then` with a callback that may return `Err`, define the initial `Result` with an explicit return type annotation. This ensures the error type is correctly inferred.
>
> ```python
> from pyrope import Ok, Err, Result
>
> def fetch_data() -> Result[int, str]:  # Declare error type here
>     return Ok(42)
>
> def validate(x: int) -> Result[int, str]:
>     return Err("invalid") if x < 0 else Ok(x)
>
> # Error type flows correctly through the chain
> result = fetch_data().and_then(validate)
> ```

### Option Type (Safe None Handling)

No more `AttributeError: 'NoneType' object has no attribute '...'`.

```python
from pyrope import Some, None_, Option

def find_user(user_id: int) -> Option[str]:
    return Some("Alice") if user_id == 1 else None_()

name_opt = find_user(1)
# You MUST check or unwrap explicitly
name = name_opt.unwrap_or("Guest")
print(f"Hello, {name}!")  # Hello, Alice!
```

## Blueprint (Batch Execution)

For performance-critical pipelines, use `Blueprint` to define a sequence of operations and execute them in a single Rust call.

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

## Syntactic Sugar: `@do` Decorator

Generator-based short-circuiting reproduces Rust's `?` operator in Python.

```python
from pyrope import Ok, Result, do

@do
def process(value: str) -> Result[str, object]:
    text = yield Ok(value)  # Type checkers infer 'text' as str
    upper = yield Ok(text.upper())
    return Ok(f"Processed: {upper}")

print(process("hello").unwrap())  # "Processed: HELLO"
```

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

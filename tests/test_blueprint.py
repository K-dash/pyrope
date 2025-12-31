from __future__ import annotations

from collections.abc import Generator

from pyrope import Blueprint, ErrorKind, Ok, Op, Result, RopeError, do, run


def test_blueprint_execution() -> None:
    bp = (
        Blueprint()
        .pipe(Op.assert_str())
        .pipe(Op.split("@"))
        .pipe(Op.index(1))
        .guard_str()
        .pipe(Op.to_uppercase())
    )

    result = run(bp, "alice@example.com")
    assert result.is_ok()
    assert result.unwrap() == "EXAMPLE.COM"

    fail_result = run(bp, "invalid-email")
    assert fail_result.is_err()
    err = fail_result.unwrap_err()
    assert str(err.kind) == str(ErrorKind.NotFound)


def test_do_with_blueprint() -> None:
    bp = Blueprint().pipe(Op.assert_str()).pipe(Op.split("@")).pipe(Op.index(1))

    @do
    def workflow(
        raw: str,
    ) -> Generator[Result[object, RopeError], object, Result[str, RopeError]]:
        domain = yield run(bp, raw)
        return Ok(f"Processed: {domain}")

    ok = workflow("alice@example.com")
    assert ok.unwrap() == "Processed: example.com"

    err = workflow("bad")
    assert err.is_err()


def test_expect_str_operator() -> None:
    """Test expect_str() as a type-narrowing operator after index/get."""
    bp = (
        Blueprint()
        .pipe(Op.assert_str())
        .pipe(Op.split("@"))
        .pipe(Op.index(1))
        .pipe(Op.expect_str())
        .pipe(Op.to_uppercase())
    )

    result = run(bp, "alice@example.com")
    assert result.is_ok()
    assert result.unwrap() == "EXAMPLE.COM"

    # Should fail if index returns non-string
    fail_bp = (
        Blueprint()
        .pipe(Op.assert_str())
        .pipe(Op.split(" "))
        .pipe(Op.index(10))  # Out of bounds -> returns error before expect_str
        .pipe(Op.expect_str())
    )
    fail_result = run(fail_bp, "hello world")
    assert fail_result.is_err()


def test_namespace_style_operators() -> None:
    """Test namespaced operators (Op.text.*, Op.seq.*, etc.)."""
    bp = (
        Blueprint()
        .pipe(Op.coerce.assert_str())  # Narrow type to str first
        .pipe(Op.text.split(","))
        .pipe(Op.seq.index(0))
        .pipe(Op.coerce.expect_str())
        .pipe(Op.text.to_uppercase())
    )

    result = run(bp, "hello,world")
    assert result.is_ok()
    assert result.unwrap() == "HELLO"


def test_namespace_and_flat_api_equivalence() -> None:
    """Test that namespace and flat API produce identical results."""
    # Namespace style
    bp_ns = (
        Blueprint()
        .pipe(Op.coerce.assert_str())  # Narrow type to str first
        .pipe(Op.text.split("@"))
        .pipe(Op.seq.index(1))
        .pipe(Op.coerce.expect_str())
        .pipe(Op.text.to_uppercase())
    )

    # Flat style (backward compatibility)
    bp_flat = (
        Blueprint()
        .pipe(Op.assert_str())  # Narrow type to str first
        .pipe(Op.split("@"))
        .pipe(Op.index(1))
        .pipe(Op.expect_str())
        .pipe(Op.to_uppercase())
    )

    input_data = "alice@example.com"
    result_ns = run(bp_ns, input_data)
    result_flat = run(bp_flat, input_data)

    assert result_ns.unwrap() == result_flat.unwrap() == "EXAMPLE.COM"


def test_len_operator_universal() -> None:
    """Test len() works on str, bytes, list, and map."""
    # String length
    bp_str = Blueprint().pipe(Op.coerce.assert_str()).pipe(Op.core.len())
    result = run(bp_str, "hello")
    assert result.is_ok()
    assert result.unwrap() == 5

    # List length
    bp_list = Blueprint().pipe(Op.core.len())
    result = run(bp_list, [1, 2, 3, 4])
    assert result.is_ok()
    assert result.unwrap() == 4

    # Map length
    result = run(bp_list, {"a": 1, "b": 2, "c": 3})
    assert result.is_ok()
    assert result.unwrap() == 3

    # Bytes length
    result = run(bp_list, b"hello world")
    assert result.is_ok()
    assert result.unwrap() == 11

    # len() should fail on unsupported types (int, bool, null)
    fail_result = run(bp_list, 42)
    assert fail_result.is_err()
    err = fail_result.unwrap_err()
    assert err.code == "type_mismatch"
    assert "str|bytes|list|map" in (err.expected or "")


def test_len_backward_compat_aliases() -> None:
    """Test that Op.len() and Op.text.len() are available as aliases."""
    bp_flat = Blueprint().pipe(Op.len())
    bp_text = Blueprint().pipe(Op.text.len())

    result_flat = run(bp_flat, "test")
    result_text = run(bp_text, "test")

    assert result_flat.unwrap() == result_text.unwrap() == 4

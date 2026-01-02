"""Tests for Option transformation methods."""

from __future__ import annotations

from pyropust import None_, Option, Some
from tests.support import SampleCode


class TestOptionMap:
    """Test Option.map() for transforming Some values."""

    def test_map_transforms_some_value(self) -> None:
        opt = Some(10).map(lambda x: x * 2)
        assert opt.is_some()
        assert opt.unwrap() == 20

    def test_map_skips_on_none(self) -> None:
        opt: Option[int] = None_().map(lambda x: x * 2)
        assert opt.is_none()


class TestOptionMapTry:
    """Test Option.map_try() for transforming Some values with exception capture."""

    def test_map_try_transforms_some_value(self) -> None:
        res = Some("123").map_try(int, code=SampleCode.ERROR, message="invalid int")
        assert res.is_ok()
        opt = res.unwrap()
        assert opt.is_some()
        assert opt.unwrap() == 123

    def test_map_try_wraps_exception(self) -> None:
        res = Some("nope").map_try(int, code=SampleCode.BAD_INPUT, message="invalid int")
        assert res.is_err()
        err = res.unwrap_err()
        assert err.code == SampleCode.BAD_INPUT
        assert err.message == "invalid int"
        assert err.cause is not None
        assert "py_exception" in err.cause

    def test_map_try_skips_on_none(self) -> None:
        res = None_().map_try(int, code=SampleCode.ERROR, message="invalid int")
        assert res.is_ok()
        opt = res.unwrap()
        assert opt.is_none()


class TestOptionMapOr:
    """Test Option.map_or() for transforming with default values."""

    def test_map_or_applies_function_on_some(self) -> None:
        opt = Some(5)
        result = opt.map_or(0, lambda x: x * 2)
        assert result == 10

    def test_map_or_returns_default_on_none(self) -> None:
        opt: Option[int] = None_()
        result = opt.map_or(0, lambda x: x * 2)
        assert result == 0


class TestOptionMapOrElse:
    """Test Option.map_or_else() for transforming with computed defaults."""

    def test_map_or_else_applies_function_on_some(self) -> None:
        opt = Some(5)
        result = opt.map_or_else(lambda: 0, lambda x: x * 2)
        assert result == 10

    def test_map_or_else_computes_default_on_none(self) -> None:
        opt: Option[int] = None_()
        result = opt.map_or_else(lambda: 42, lambda x: x * 2)
        assert result == 42


class TestOptionInspect:
    """Test Option.inspect() for side effects."""

    def test_inspect_calls_function_on_some(self) -> None:
        called: list[int] = []
        opt = Some(10)
        result = opt.inspect(lambda x: called.append(x))
        assert called == [10]
        assert result.is_some()
        assert result.unwrap() == 10

    def test_inspect_does_not_call_on_none(self) -> None:
        called: list[int] = []
        opt: Option[int] = None_()
        result = opt.inspect(lambda x: called.append(x))
        assert called == []
        assert result.is_none()


class TestOptionFilter:
    """Test Option.filter() for conditional filtering."""

    def test_filter_keeps_value_when_predicate_matches(self) -> None:
        opt = Some(10)
        result = opt.filter(lambda x: x > 5)
        assert result.is_some()
        assert result.unwrap() == 10

    def test_filter_returns_none_when_predicate_fails(self) -> None:
        opt = Some(3)
        result = opt.filter(lambda x: x > 5)
        assert result.is_none()

    def test_filter_returns_none_on_none(self) -> None:
        opt: Option[int] = None_()
        result = opt.filter(lambda x: x > 5)
        assert result.is_none()


class TestOptionAndThenTry:
    """Test Option.and_then_try() for chaining with exception capture."""

    def test_and_then_try_transforms_some_value(self) -> None:
        res = Some("123").and_then_try(
            lambda x: Some(int(x) * 2),
            code=SampleCode.ERROR,
            message="invalid int",
        )
        assert res.is_ok()
        opt = res.unwrap()
        assert opt.is_some()
        assert opt.unwrap() == 246

    def test_and_then_try_wraps_exception(self) -> None:
        res = Some("nope").and_then_try(
            lambda x: Some(int(x) * 2),
            code=SampleCode.BAD_INPUT,
            message="invalid int",
        )
        assert res.is_err()
        err = res.unwrap_err()
        assert err.code == SampleCode.BAD_INPUT
        assert err.message == "invalid int"
        assert err.cause is not None
        assert "py_exception" in err.cause

    def test_and_then_try_skips_on_none(self) -> None:
        res = None_().and_then_try(
            lambda x: Some(x),
            code=SampleCode.ERROR,
            message="invalid int",
        )
        assert res.is_ok()
        opt = res.unwrap()
        assert opt.is_none()

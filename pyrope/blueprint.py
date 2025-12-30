from __future__ import annotations

from collections.abc import Mapping, Sequence

from .pyrope_native import (
    Operator,
    _op_assert_str,
    _op_get_key,
    _op_index,
    _op_split,
    _op_to_uppercase,
)


class Op:
    @staticmethod
    def assert_str() -> Operator[object, str]:
        return _op_assert_str()

    @staticmethod
    def expect_str() -> Operator[object, str]:
        """Type-narrowing operator that asserts the input is a string.

        Alias for assert_str() for use after operations that return object.

        Usage:
            Blueprint().pipe(Op.index(0)).pipe(Op.expect_str()).pipe(Op.to_uppercase())
        """
        return _op_assert_str()

    @staticmethod
    def split(delim: str) -> Operator[str, list[str]]:
        return _op_split(delim)

    @staticmethod
    def index(idx: int) -> Operator[Sequence[object], object]:
        return _op_index(idx)

    @staticmethod
    def get(key: str) -> Operator[Mapping[str, object], object]:
        return _op_get_key(key)

    @staticmethod
    def to_uppercase() -> Operator[str, str]:
        return _op_to_uppercase()

from __future__ import annotations

import importlib.util
import sys
from pathlib import Path
from typing import Any, cast

import pytest


def _import_gen_ops() -> Any:
    """Import tools/gen_ops.py as a module (tools/ is not a package)."""
    root = Path(__file__).resolve().parents[1]
    gen_ops_path = root / "tools" / "gen_ops.py"

    spec = importlib.util.spec_from_file_location("gen_ops", gen_ops_path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"Failed to load module spec for {gen_ops_path}")

    module = importlib.util.module_from_spec(spec)
    # Avoid cross-test contamination if the module was previously loaded.
    sys.modules.pop("gen_ops", None)
    spec.loader.exec_module(module)
    return cast("Any", module)


def test_parse_kind_rs_extracts_specs(tmp_path: Path) -> None:
    gen_ops = _import_gen_ops()

    kind_rs = tmp_path / "kind.rs"
    kind_rs.write_text(
        """#[derive(Clone, Debug)]
pub enum OperatorKind {
    /// @op name=split py=split
    /// @sig in=str out=list[str]
    /// @param delim:str
    Split { delim: String },

    /// @op name=index py=index
    /// @sig in=Sequence[object] out=object
    /// @param idx:int
    Index { idx: usize },
}
"""
    )

    specs = cast("list[Any]", gen_ops.parse_kind_rs(kind_rs))
    assert [s.py_name for s in specs] == ["split", "index"]

    split = specs[0]
    assert split.variant_name == "Split"
    assert split.in_type == "str"
    assert split.out_type == "list[str]"
    assert [(p.name, p.py_type, p.rust_type) for p in split.params] == [
        ("delim", "str", "String"),
    ]

    index = specs[1]
    assert index.variant_name == "Index"
    assert index.in_type == "Sequence[object]"
    assert index.out_type == "object"
    assert [(p.name, p.py_type, p.rust_type) for p in index.params] == [
        ("idx", "int", "usize"),
    ]


def test_parse_kind_rs_handles_complex_sig_types(tmp_path: Path) -> None:
    gen_ops = _import_gen_ops()

    kind_rs = tmp_path / "kind.rs"
    kind_rs.write_text(
        """pub enum OperatorKind {
    /// @op name=get py=get
    /// @sig in=Mapping[str, object] out=object
    /// @param key:str
    GetKey { key: String },
}
"""
    )

    (spec,) = cast("list[Any]", gen_ops.parse_kind_rs(kind_rs))
    assert spec.in_type == "Mapping[str, object]"
    assert spec.out_type == "object"


def test_generate_rust_op_contains_expected_methods() -> None:
    gen_ops = _import_gen_ops()

    spec_no_params = gen_ops.OpSpec(
        name="assert_str",
        py_name="assert_str",
        in_type="object",
        out_type="str",
        params=[],
        variant_name="AssertStr",
    )
    spec_with_params = gen_ops.OpSpec(
        name="split",
        py_name="split",
        in_type="str",
        out_type="list[str]",
        params=[gen_ops.OpParam(name="delim", py_type="str", rust_type="String")],
        variant_name="Split",
    )

    rust = cast("str", gen_ops.generate_rust_op([spec_no_params, spec_with_params]))

    assert "pub fn assert_str() -> Operator" in rust
    assert "kind: OperatorKind::AssertStr," in rust

    assert "pub fn split(delim: String) -> Operator" in rust
    assert "kind: OperatorKind::Split { delim }," in rust


def test_generate_python_stub_and_update_replaces_marked_region(tmp_path: Path) -> None:
    gen_ops = _import_gen_ops()

    specs = [
        gen_ops.OpSpec(
            name="len",
            py_name="len",
            in_type="str",
            out_type="int",
            params=[],
            variant_name="Len",
        )
    ]

    new_stub = cast("str", gen_ops.generate_python_stub(specs))
    # Generated stub now has no indentation (indentation is added during replacement)
    assert new_stub.startswith("# BEGIN GENERATED OP")
    assert "def len() -> Operator[str, int]:" in new_stub
    assert new_stub.rstrip().endswith("# END GENERATED OP")

    pyi = tmp_path / "__init__.pyi"
    pyi.write_text(
        """class Operator: ...

class Op:
    # BEGIN GENERATED OP
    @staticmethod
    def old() -> Operator: ...
    # END GENERATED OP

def Ok(): ...
"""
    )

    gen_ops.update_python_stub(pyi, new_stub)

    updated = pyi.read_text()
    assert "def old()" not in updated
    assert "def len() -> Operator[str, int]:" in updated
    # Ensure surrounding content remains.
    assert "class Operator: ..." in updated
    assert "def Ok(): ..." in updated


def test_update_python_stub_raises_without_markers(tmp_path: Path) -> None:
    gen_ops = _import_gen_ops()

    pyi = tmp_path / "__init__.pyi"
    pyi.write_text("class Op:\n    pass\n")

    with pytest.raises(ValueError, match="Markers not found"):
        gen_ops.update_python_stub(pyi, "    # BEGIN GENERATED OP\n    # END GENERATED OP")

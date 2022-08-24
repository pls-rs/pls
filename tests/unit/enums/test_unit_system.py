import pytest

from pls.enums.unit_system import UnitSystem


@pytest.mark.parametrize(
    "unit_system, base, pad",
    [
        (UnitSystem.DECIMAL, 1000, 2),
        (UnitSystem.BINARY, 1024, 3),
        (UnitSystem.NONE, 1, 0),
    ],
)
def test_unit_system_has_correct_base_pad(unit_system: UnitSystem, base: int, pad: int):
    us_base, us_pad, _ = unit_system.base_pad_units
    assert base == us_base
    assert pad == us_pad


def test_binary_system_units_have_correct_prefix():
    _, _, units = UnitSystem.BINARY.base_pad_units
    assert all([unit.endswith("i") for unit in units[1:]])


def test_decimal_system_units_have_correct_prefix():
    _, _, units = UnitSystem.DECIMAL.base_pad_units
    assert not any([unit.endswith("i") for unit in units[1:]])


def test_none_system_units_are_empty():
    _, _, units = UnitSystem.NONE.base_pad_units
    assert units == [""]

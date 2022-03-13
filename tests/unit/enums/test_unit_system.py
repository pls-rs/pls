from pls.enums.unit_system import UnitSystem, get_base_and_pad_and_units


def test_binary_system():
    base, _, units = get_base_and_pad_and_units(UnitSystem.BINARY)
    assert base == 1024
    assert all([unit.endswith("i") for unit in units[1:]])


def test_decimal_system():
    base, _, units = get_base_and_pad_and_units(UnitSystem.DECIMAL)
    assert base == 1000
    assert not any([unit.endswith("i") for unit in units[1:]])

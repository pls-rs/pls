from typing import Literal, TypedDict


class ColumnAttrs(TypedDict, total=False):
    justify: Literal["left", "right"]
    width: int


class ColumnSpec(TypedDict, total=False):
    name: str
    attrs: ColumnAttrs

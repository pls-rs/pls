from typing import Literal, TypedDict


class ColumnAttrs(TypedDict, total=False):
    """
    This model defines attributes associated with the column that are used by
    Rich to render the table.
    """

    justify: Literal["left", "right"]
    width: int


class ColumnSpec(TypedDict, total=False):
    """
    This model defines the readable name and Rich-specific attributes of the
    column.
    """

    name: str
    attrs: ColumnAttrs

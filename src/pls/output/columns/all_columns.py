from __future__ import annotations

from pls.config import constants
from pls.enums.icon_type import IconType
from pls.globals import args
from pls.models.column_spec import ColumnSpec
from pls.output.columns.detail_columns import detail_column_groups, detail_column_specs


column_specs: dict[str, ColumnSpec] = {
    "spacer": ColumnSpec(
        key="spacer",
        name=constants.constants.lookup(["spacer", "name"]),
        value=constants.constants.lookup(["spacer", "value"]),
    ),  # dummy column to act like spacer
    **detail_column_specs,
    "icon": ColumnSpec(
        key="icon",
        name="",  # pseudo-column,
        attrs={"width": 2},
        is_available=(lambda: args.args.icon != IconType.NONE),
    ),
    "name": ColumnSpec(key="name", name=" Name" if args.args.align else "Name"),
}
"""a mapping of column keys to column specs"""

column_groups = detail_column_groups, [["icon", "name"]]
"""list of list of column names that are placed together without spacers"""

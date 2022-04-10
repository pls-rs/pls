from typing import Callable, Literal, Optional, TypedDict, Union


class ColumnAttrs(TypedDict, total=False):
    """
    This model defines attributes associated with the column that are used by
    Rich to render the table.
    """

    justify: Literal["left", "right"]
    width: int


class ColumnSpec:
    """
    This model defines the readable name and Rich-specific attributes of the
    column.
    """

    def __init__(
        self,
        key: str,
        name: str,
        value: Optional[str] = None,
        is_available: Optional[Union[Callable[[], bool], bool]] = None,
        attrs: Optional[ColumnAttrs] = None,
    ):
        """
        Instantiate a new column specification.

        :param key: the key used to internally refer to this column
        :param name: the readable name of the column, used in the header
        :param value: the value of the column to use in all non-header rows
        :param is_available: the conditions for the column to be visible
        """

        self.key = key
        self.name = name

        self.value = value
        self._is_available = is_available if is_available is not None else True
        self.attrs = attrs if attrs is not None else {}

    @property
    def is_available(self):
        """
        Get the computed value of the ``_is_available`` attribute, which can be a
        boolean value or a callable without any arguments that evaluates to a boolean
        value.

        :return: the computed value of the object's ``_is_available`` property
        """

        is_avail = self._is_available
        return is_avail() if callable(is_avail) else is_avail

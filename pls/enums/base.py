from enum import Enum


class AutoEnum(Enum):
    def _generate_next_value_(self: str, *args) -> str:
        return self.lower()

    def __str__(self):
        """
        Get the string representation of this enum. Unlike other objects, this
        does not default to ``__repr__``.
        :return: the string representation
        """
        return self.value

from enum import Enum


class AutoEnum(Enum):
    @staticmethod
    def _generate_next_value_(name: str, *args) -> str:
        return name.lower()

    def __str__(self):
        """
        Get the string representation of this enum. Unlike other objects, this
        does not default to ``__repr__``.
        :return: the string representation
        """
        return self.value

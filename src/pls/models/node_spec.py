from __future__ import annotations

import re
from typing import TYPE_CHECKING


if TYPE_CHECKING:
    from pls.models.base_node import BaseNode


class NodeSpec:
    """
    A spec describes the nature of a class of FS nodes that determine its
    presentation. This model stores attributes pertaining to a single spec. Make
    sure to massage spec entries using :py:func:`pls.data.config.massage_specs`
    before initialising.

    Node specs are read from ``node_spec.yml``.
    """

    def __init__(
        self,
        name: str = None,
        pattern: str = None,
        glob: str = None,
        extension: str = None,
        icon: str = None,
        color: str = None,
        importance: int = 0,
        collapse: dict = None,
        **kwargs,  # Ignore all unwanted arguments
    ):
        self.name = name
        self.pattern = re.compile(pattern) if pattern else None
        self.glob = glob
        self.extension = extension

        self.icon = icon
        self.color = color
        self.importance = importance
        self.collapse = collapse

    def __repr__(self) -> str:
        """
        Get the string representation of the ``NodeSpec`` instance. This is also
        used by ``__str__`` automatically.

        :return: the string representation
        """

        if self.name:
            return self.name
        if self.extension:
            return f"*.{self.extension}"
        if self.pattern:
            return f"/{self.pattern.pattern}/"
        if self.glob:
            return f"<{self.glob}>"
        return "[No ID]"

    def match(self, node: BaseNode) -> bool:
        """
        Check whether the given node matches this spec. The criterion for
        evaluating a match is based on whether the spec defines the name,
        regular expression, glob pattern or extension.

        :param node: the node to compare against the spec for a match
        :return: ``True`` if the node matches this entry, ``False`` otherwise
        """

        if self.name:
            return self.name == node.name
        elif self.pattern:
            return self.pattern.match(node.name) is not None
        elif self.glob:
            return node.path.match(self.glob)
        elif self.extension:
            return self.extension == node.extension
        else:
            return False

import re
from typing import Optional
from unittest.mock import patch

import pytest

from pls.fs.list import passes_name_filters
from pls.globals import args


@pytest.mark.parametrize(
    "exclude, only, expectation",
    [
        (r"README", None, False),
        (r".*\.md", None, False),
        (None, r"readme", False),
        (None, r".*\.py", False),
        (r"readme", r".*\.md", True),
        (None, None, True),
    ],
)
def test_filtering_matches_both_exclude_and_only(
    exclude: Optional[str], only: Optional[str], expectation: bool
):
    exclude_re = re.compile(exclude) if exclude is not None else None
    only_re = re.compile(only) if only is not None else None
    with patch.multiple(args.args, exclude=exclude_re, only=only_re):
        assert passes_name_filters("README.md") == expectation

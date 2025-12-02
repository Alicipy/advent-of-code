import pytest

from aoc2025.day02.main import is_invalid_id


@pytest.mark.parametrize("id", ["11", "22"])
def test_is_invalid_id__several_invalid_ids__detected_invalid(invalid_id):

    assert is_invalid_id(invalid_id)

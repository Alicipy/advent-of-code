import pytest

from aoc2025.day02.main import part_1, part_2, is_invalid_id_1, is_invalid_id_2

TEST_INPUT = (
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,"
    "1698522-1698528,446443-446449,38593856-38593862,"
    "565653-565659,824824821-824824827,2121212118-2121212124"
)


def test__part_1__example():
    assert part_1([TEST_INPUT]) == 1227775554


def test__part_2__example():
    assert part_2([TEST_INPUT]) == 4174379265


@pytest.mark.parametrize("invalid_id", ["11", "22", "1010"])
def test_is_invalid_id_1_several_invalid_ids__detected_invalid(invalid_id):
    assert is_invalid_id_1(invalid_id)


@pytest.mark.parametrize("valid_id", ["12", "13", "1698522"])
def test_is_invalid_id_1_no_invalid_ids__detected_valid(valid_id):
    assert is_invalid_id_1(valid_id) == False


@pytest.mark.parametrize("invalid_id", ["38593859"])
def test_is_invalid_id_2_invalid_id__detected_invalid(invalid_id):
    assert is_invalid_id_2(invalid_id)

from fimficdata.common.util import next_nth_minute
from random import randint
import pytest


@pytest.mark.parametrize(
    'in_interval,in_time',
    [(1, None),
     (2, None),
     (3, None),
     (4, None),
     (5, None),
     (6, None),
     (7, None),
     (8, None),
     (9, None),
     (10, None),
     (11, None),
     (12, None),
     (13, None),
     (15, None),
     (20, None),
     (30, None),
     (59, None),
     (60, None),
     (61, None)])
def test_default_time_divides_into_minutes(in_interval, in_time):
    assert next_nth_minute(in_interval, in_time) % (in_interval * 60) == 0


@pytest.mark.parametrize(
    'in_interval,in_time',
    [(1, 1641061023),
     (2, 1641061023),
     (3, 1641061023),
     (4, 1641061023),
     (5, 1641061023),
     (6, 1641061023),
     (7, 1641061023),
     (8, 1641061023),
     (9, 1641061023),
     (10, 1641061023),
     (11, 1641061023),
     (12, 1641061023),
     (13, 1641061023),
     (15, 1641061023),
     (20, 1641061023),
     (30, 1641061023),
     (59, 1641061023),
     (60, 1641061023),
     (61, 1641061023)])
def test_specific_time_divides_into_minutes(in_interval, in_time):
    assert next_nth_minute(in_interval, in_time) % (in_interval * 60) == 0


@pytest.mark.parametrize(
    'in_interval,in_time,out_time',
    [(1, 1641061023, 1641061080),
     (1, 1640995199, 1640995200),
     (5, 1641061023, 1641061200),
     (5, 1641061267, 1641061500),
     (5, 1641061199, 1641061200),
     (15, 1641061023, 1641061800),
     (15, 1641060792, 1641060900),
     (15, 1641060899, 1641060900),
     (30, 1641061023, 1641061800),
     (30, 1641061858, 1641063600),
     (30, 1641061799, 1641061800),
     (60, 1641061023, 1641063600),
     (120, 1641061023, 1641067200)])
def test_specific_time_equals_specific_time(in_interval, in_time, out_time):
    assert next_nth_minute(in_interval, in_time) == out_time


@pytest.mark.parametrize(
    'in_interval,in_time,out_time',
    [(1, 1641061020, 1641061080),
     (5, 1641061200, 1641061500),
     (10, 1641061200, 1641061800),
     (15, 1641061800, 1641062700),
     (30, 1641061800, 1641063600),
     (60, 1641063600, 1641067200),
     (120, 1641067200, 1641074400)])
def test_already_at_nth_minute(in_interval, in_time, out_time):
    assert next_nth_minute(in_interval, in_time) == out_time


# TODO: Unsure if this should be parametrized. Random testing data is bad?
def test_random_time_equals_whole_minute():
    data = [(randint(1, 500), randint(0, 2000000000)) for _ in range(30)]

    for datum in data:
        assert next_nth_minute(datum[0], datum[1]) % (datum[0] * 60) == 0


@pytest.mark.parametrize(
    'in_interval,in_time',
    [('a',  1641061023),
     (0,    1641061023),
     (-1,   1641061023),
     (True, 1641061023),
     (1,    False),
     (1,    'abc')])
def test_invalid_arguments(in_interval, in_time):
    with pytest.raises(ValueError):
        next_nth_minute(in_interval, in_time)
